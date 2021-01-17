// For processing image thumbnails in parallel

// Task dependencies
use crate::data::content::{ImageMetadata, PathThumb};
// use async_std::future;
// use async_std::prelude::*;
// use async_std::task;
use async_std::task::{Context, Poll};
use derivative::Derivative;
use futures::stream::{FusedStream, Stream, StreamExt, TryStream, TryStreamExt};
use futures::FutureExt;
use iced::Subscription;
use iced_futures::futures;
use image::codecs::jpeg::JpegEncoder;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};
use log::{debug, error};
// use par_stream::ParStreamExt;
use std::future::Future;
use std::hash::Hash;
use std::path::PathBuf;
use std::pin::Pin;
use std::time::{Duration, Instant};

// What is needed to create this task?
pub struct ProcessThumbs {
    start: Instant,
    paths: Vec<PathBuf>,
}

// Size in bytes (max value: 18.45 exabytes)
// type Size = u64;

// #[derive(Debug, Clone)]
// struct Perf {
//     bytes: Size,
//     elapsed: Duration,
// }

// For task output
#[derive(Debug, Clone, Hash)]
pub enum Progress {
    Ready(PathBuf),
    Finished(PathThumb, Duration),
    Error(String),
}

const THUMB_SIZE: f32 = 256.0;

// Utility function
pub fn process_paths(paths: Vec<PathBuf>) -> iced::Subscription<Progress> {
    let start = Instant::now();
    debug!("Processing {} paths", paths.len());
    Subscription::from_recipe(ProcessThumbs { paths, start })
}

fn resize_image(path: &PathBuf) -> Option<(DynamicImage, u32, u32)> {
    let path = path.clone();

    match ImageReader::open(&path).unwrap().decode() {
        Ok(img) => {
            let (width, height) = img.dimensions();
            let aspect_ratio = height as f32 / width as f32;
            let width = THUMB_SIZE;
            let height = THUMB_SIZE * aspect_ratio;
            let width = width as u32;
            let height = height.round() as u32;
            Some((img.thumbnail_exact(width, height), width, height))
        }
        Err(err) => {
            error!(
                "Error decoding image at path: {:?}\nError was: {}",
                &path, err
            );
            None
        }
    }
}

// ParStream Config
pub trait IntoParStreamParams {
    fn into_par_stream_params(self) -> ParStreamParams;
}

impl<T> IntoParStreamParams for T
where
    ParStreamConfig: From<T>,
{
    fn into_par_stream_params(self) -> ParStreamParams {
        let config: ParStreamConfig = self.into();
        let params: ParStreamParams = config.into();
        params
    }
}

/// Parallel stream configuration.
#[derive(Debug, Clone)]
pub struct ParStreamConfig {
    pub num_workers: Value,
    pub buf_size: Value,
}

impl From<(usize, usize)> for ParStreamConfig {
    fn from((num_workers, buf_size): (usize, usize)) -> Self {
        ParStreamConfig {
            num_workers: Value::Absolute(num_workers),
            buf_size: Value::Absolute(buf_size),
        }
    }
}

/// Sum type of absolute value and scaling value.
#[derive(Debug, Clone)]
pub enum Value {
    Auto,
    Absolute(usize),
    Scale(f64),
}

impl Value {
    pub fn to_absolute(&self) -> usize {
        match *self {
            Self::Auto => num_cpus::get(),
            Self::Absolute(val) => {
                assert!(val > 0, "absolute value must be positive");
                val
            }
            Self::Scale(scale) => {
                assert!(
                    scale.is_finite() && scale.is_sign_positive(),
                    "scaling value must be positive finite"
                );
                (num_cpus::get() as f64 * scale).ceil() as usize
            }
        }
    }
}

/// Parallel stream parameters.
#[derive(Debug, Clone)]
pub struct ParStreamParams {
    pub(crate) num_workers: usize,
    pub(crate) buf_size: usize,
}

impl From<ParStreamConfig> for ParStreamParams {
    fn from(from: ParStreamConfig) -> Self {
        let ParStreamConfig {
            num_workers,
            buf_size,
        } = from;

        let num_workers = num_workers.to_absolute();
        let buf_size = buf_size.to_absolute();

        Self {
            num_workers,
            buf_size,
        }
    }
}

impl From<Option<usize>> for ParStreamConfig {
    fn from(size: Option<usize>) -> Self {
        match size {
            Some(size) => ParStreamConfig {
                num_workers: Value::Absolute(size),
                buf_size: Value::Absolute(size),
            },
            None => ParStreamConfig {
                num_workers: Value::Auto,
                buf_size: Value::Auto,
            },
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ParMapUnordered<T> {
    #[derivative(Debug = "ignore")]
    fut: Option<Pin<Box<dyn Future<Output = ((), Vec<()>)> + Send>>>,
    #[derivative(Debug = "ignore")]
    output_rx: async_std::channel::Receiver<T>,
}

impl<T> ParMapUnordered<T> {
    fn new<S, F, Fut>(mut stream: S, config: impl IntoParStreamParams, mut f: F) -> Self
    where
        T: 'static + Send,
        F: 'static + FnMut(S::Item) -> Fut + Send,
        Fut: 'static + Future<Output = T> + Send,
        S: 'static + StreamExt + Sized + Unpin + Send,
        S::Item: Send,
    {
        let ParStreamParams {
            num_workers,
            buf_size,
        } = config.into_par_stream_params();
        let (map_tx, map_rx) = async_std::channel::bounded(buf_size);
        let (output_tx, output_rx) = async_std::channel::bounded(buf_size);

        let map_fut = async move {
            while let Some(item) = stream.next().await {
                let fut = f(item);
                map_tx.send(fut).await.unwrap();
            }
        };

        let worker_futs: Vec<_> = (0..num_workers)
            .map(|_| {
                let map_rx = map_rx.clone();
                let output_tx = output_tx.clone();

                let worker_fut = async move {
                    while let Ok(fut) = map_rx.recv().await {
                        let output = fut.await;
                        println!("HALP");
                        output_tx.send(output).await.unwrap();
                    }
                };
                let worker_fut = async_std::task::spawn(worker_fut);
                worker_fut
            })
            .collect();

        let par_then_fut = futures::future::join(map_fut, futures::future::join_all(worker_futs));

        Self {
            fut: Some(Box::pin(par_then_fut)),
            output_rx,
        }
    }
}

impl<T> Stream for ParMapUnordered<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let mut should_wake = match self.fut.as_mut() {
            Some(fut) => match Pin::new(fut).poll(cx) {
                Poll::Pending => true,
                Poll::Ready(_) => {
                    self.fut = None;
                    false
                }
            },
            None => false,
        };

        let poll = Pin::new(&mut self.output_rx).poll_next(cx);
        should_wake |= !self.output_rx.is_empty();

        if should_wake {
            cx.waker().wake_by_ref();
        }

        poll
    }
}

// Par stream async-std
pub trait ParStreamExt {
    fn par_map_unordered<T, F, Func>(
        self,
        config: impl IntoParStreamParams,
        mut f: F,
    ) -> ParMapUnordered<T>
    where
        T: 'static + Send,
        F: 'static + FnMut(Self::Item) -> Func + Send,
        Func: 'static + FnOnce() -> T + Send,
        Self: 'static + StreamExt + Sized + Unpin + Send,
        Self::Item: Send,
    {
        self.par_then_unordered(config, move |item| {
            let func = f(item);
            async_std::task::spawn_blocking(func)
        })
    }

    fn par_then_unordered<T, F, Fut>(
        self,
        config: impl IntoParStreamParams,
        f: F,
    ) -> ParMapUnordered<T>
    where
        T: 'static + Send,
        F: 'static + FnMut(Self::Item) -> Fut + Send,
        Fut: 'static + Future<Output = T> + Send,
        Self: 'static + StreamExt + Sized + Unpin + Send,
        Self::Item: Send,
    {
        ParMapUnordered::new(self, config, f)
    }
}

impl<S> ParStreamExt for S where S: Stream {}

// Task implementation
impl<H, I> iced_native::subscription::Recipe<H, I> for ProcessThumbs
where
    H: std::hash::Hasher,
{
    type Output = Progress;

    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        self.paths.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        let start = self.start.clone();

        let stream = futures::stream::iter(self.paths)
            .inspect(|x| println!("made it through filter 1"))
            .par_map_unordered(None, move |path| {
                move || {
                    let image_result = resize_image(&path);

                    if let Some((decoded_image, width_px, height_px)) = image_result {
                        let mut output = Box::new(vec![]);
                        let mut encoder = JpegEncoder::new(&mut output);
                        encoder.encode_image(&decoded_image).unwrap();

                        let size_bytes = output.len() as u64;
                        let mime_type = "image/jpeg".to_string();

                        let metadata = ImageMetadata {
                            size_bytes,
                            mime_type,
                            width_px,
                            height_px,
                        };

                        let image = output.into_boxed_slice();

                        debug!("Processing {:.2?}", &start.elapsed());

                        Progress::Finished(
                            PathThumb {
                                path,
                                image,
                                metadata,
                            },
                            start.elapsed(),
                        )
                    } else {
                        let error = format!(
                            "Error decoding image after {:.2?}, at: {:?}",
                            &start.elapsed(),
                            &path,
                        );
                        Progress::Error(error)
                    }
                }
            });

        // let (output_tx, output_rx) = stream.split();

        stream
            .inspect(|x| println!("made it through filter 2"))
            .boxed()

        // let (output_tx, output_rx) = async_std::channel::unbounded();
    }
}
