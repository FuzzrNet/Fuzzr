// For processing image thumbnails in parallel

// Task dependencies
use crate::data::content::{ImageMetadata, PathThumb};
// use async_std::future;
// use async_std::prelude::*;
// use async_std::task;
use async_std::channel::{Receiver, Sender};
use async_std::sync::{Arc, Mutex};
use async_std::task::{Context, Poll};
use derivative::Derivative;
use futures::stream::{self, FusedStream, Stream, StreamExt, TryStream, TryStreamExt};
use futures::FutureExt;
use iced::Subscription;
use iced_futures::futures;
use image::codecs::jpeg::JpegEncoder;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};
use log::{debug, error, info};
use par_stream::ParStreamExt;
use std::future::Future;
use std::hash::Hash;
use std::path::PathBuf;
use std::pin::Pin;
use std::time::{Duration, Instant};

// Size in bytes (max value: 18.45 exabytes)
// type Size = u64;

// #[derive(Debug, Clone)]
// struct Perf {
//     bytes: Size,
//     elapsed: Duration,
// }

// What is needed to create this task?
pub struct ProcessThumbs {
    time_started: Instant,
    paths: Vec<PathBuf>,
}

// For task output
#[derive(Debug, Clone)]
pub enum Progress {
    Updated {
        time_elapsed: Duration,
        total_paths: usize,
        thumb: PathThumb,
    },
    Finished {
        time_elapsed: Duration,
        error: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum State {
    Updated {
        time_started: Instant,
        thumb_rx: Receiver<Option<PathThumb>>,
    },
    Finished,
}

const THUMB_SIZE: f32 = 256.0;

// Utility function
pub fn process_paths(paths: Vec<PathBuf>) -> iced::Subscription<Progress> {
    debug!("Processing {} paths", paths.len());
    let time_started = Instant::now();
    Subscription::from_recipe(ProcessThumbs {
        paths,
        time_started,
    })
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

// Task implementation
impl<H, I> iced_native::subscription::Recipe<H, I> for ProcessThumbs
where
    H: std::hash::Hasher,
{
    type Output = Progress;

    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        self.time_started.hash(state);
        self.paths.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        let total_paths = self.paths.len();
        let path_stream = Arc::new(Mutex::new(futures::stream::iter(self.paths)));
        let num_workers = num_cpus::get();
        let time_started = self.time_started.clone();

        let (map_tx, map_rx) = async_std::channel::unbounded();
        let (thumb_tx, thumb_rx) = async_std::channel::unbounded();

        let map_fut = async move {
            let path_stream = &mut path_stream.lock().await;
            while let Some(path) = path_stream.next().await {
                let fut = async_std::task::spawn_blocking(move || {
                    let image_result = resize_image(&path);

                    if let Some((decoded_image, width_px, height_px)) = resize_image(&path) {
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

                        debug!("Processing {:.2?}", &time_started.elapsed());

                        Some(PathThumb {
                            path: path.clone(),
                            image,
                            metadata,
                        })
                    } else {
                        None
                    }
                });

                map_tx.send(fut).await.unwrap();
            }
        };

        let worker_futs: Vec<_> = (0..num_workers)
            .map(|_| {
                let map_rx = map_rx.clone();
                let thumb_tx = thumb_tx.clone();

                let worker_fut = async move {
                    while let Ok(fut) = map_rx.recv().await {
                        let output = fut.await;
                        println!("issue {:?}", output);
                        thumb_tx.send(output).await.unwrap();
                    }
                };
                let worker_fut = async_std::task::spawn(worker_fut);
                worker_fut
            })
            .collect();

        let par_then_fut = futures::future::join(map_fut, futures::future::join_all(worker_futs));

        async_std::task::spawn(par_then_fut);

        futures::stream::unfold(
            State::Updated {
                time_started: self.time_started,
                thumb_rx,
            },
            move |state| async move {
                match state {
                    State::Updated {
                        time_started,
                        thumb_rx,
                    } => match thumb_rx.recv().await {
                        Ok(next_val) => {
                            if let Some(thumb) = next_val {
                                debug!("Processing {:.2?}", &time_started.elapsed());

                                Some((
                                    Progress::Updated {
                                        time_elapsed: time_started.elapsed(),
                                        total_paths: total_paths.clone(),
                                        thumb,
                                    },
                                    State::Updated {
                                        time_started,
                                        thumb_rx,
                                    },
                                ))
                            } else {
                                let error = format!(
                                    "Error decoding image after {:.2?}",
                                    &time_started.elapsed(),
                                );

                                Some((
                                    Progress::Finished {
                                        time_elapsed: time_started.elapsed(),
                                        error: Some(error),
                                    },
                                    State::Finished,
                                ))
                            }
                        }
                        Err(err) => Some((
                            Progress::Finished {
                                time_elapsed: time_started.elapsed(),
                                error: None,
                            },
                            State::Finished,
                        )),
                    },
                    State::Finished => {
                        // We do not let the stream die, as it would start a
                        // new download repeatedly if the user is not careful
                        // in case of errors.
                        let _: () = iced::futures::future::pending().await;

                        None
                    }
                }
            },
        )
        .boxed()
    }
}
