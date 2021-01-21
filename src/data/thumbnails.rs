// For processing image thumbnails in parallel

// Task dependencies
use crate::data::content::{ImageMetadata, PathThumb};
// use async_std::future;
// use async_std::prelude::*;
// use async_std::task;
// use async_std::channel::{Receiver, Sender};
// use async_std::stream::StreamExt;
// use async_std::task::{Context, Poll};
use derivative::Derivative;
use futures::future::BoxFuture;
// use futures::stream::{
//     self, FusedStream, IntoStream, Scan, Stream, StreamExt, TryStream, TryStreamExt,
// };
use futures::prelude::*;
// use futures::{FutureExt, SinkExt};
// use iced::futures::stream::Scan;
use iced::Subscription;
use iced_futures::futures;
// use iced_futures::futures::{Stream, StreamExt};
use image::codecs::jpeg::JpegEncoder;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};
use log::{debug, error, info};
// use parallel_stream::prelude::*;
use crossbeam_deque::{Injector, Steal, Stealer, Worker};
use crossbeam_utils::atomic::AtomicCell;
use std::borrow::Borrow;
use std::future::Future;
use std::hash::Hash;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::sync::Mutex;

// use rayon::iter::ParallelBridge;
// use rayon::prelude::ParallelIterator;
// use rayon::prelude::*;

// use async_std::prelude::*;
// use async_std::stream;

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
        total_paths: usize,
        // receiver: Arc<Mutex<mpsc::Receiver<PathThumb>>>,
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

fn find_task<T>(local: &Worker<T>, global: &Injector<T>, stealers: &[Stealer<T>]) -> Option<T> {
    // Pop a task from the local queue, if not empty.
    local.pop().or_else(|| {
        // Otherwise, we need to look for a task elsewhere.
        std::iter::repeat_with(|| {
            // Try stealing a batch of tasks from the global queue.
            global
                .steal_batch_and_pop(local)
                // Or try stealing a task from one of the other threads.
                .or_else(|| stealers.iter().map(|s| s.steal()).collect())
        })
        // Loop while no task was stolen and any steal operation needs to be retried.
        .find(|s| !s.is_retry())
        // Extract the stolen task, if there is one.
        .and_then(|s| s.success())
    })
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
        // let paths = self.paths.clone().into_iter();
        let total_paths = self.paths.len();
        let time_started_local = self.time_started.clone();
        let num_workers = num_cpus::get();

        let deque = Injector::new();

        // self.paths.iter().for_each(|p| {
        //     q.push(p);
        // });

        // let subscription_stream = stream::FuturesUnordered::new();
        let thumb_rx_stream = stream::iter(self.paths).map(move |path| {
            let (thumb_tx, thumb_rx) = tokio::sync::oneshot::channel::<PathThumb>();
            deque.push((path.clone(), thumb_tx));

            (time_started_local, total_paths, thumb_rx)
        });

        // let mut worker_handles: Vec<tokio::task::JoinHandle<Progress>> = vec![];

        // let (worker_sender, mut worker_receiver) =
        //     tokio::sync::mpsc::channel::<PathThumb>(num_workers);
        // let receiver = Arc::new(tokio::sync::Mutex::new(receiver));

        let thumbs_processed = Arc::new(AtomicCell::new(0usize));

        let stealers = (0..num_workers).map(|_| {
            let w: Worker<(PathBuf, tokio::sync::oneshot::Sender<PathThumb>)> = Worker::new_lifo();
            w.stealer()
        });

        let worker_handles = stealers.map(|s| {
            let thumbs_processed = Arc::clone(&thumbs_processed);
            println!("hello 1");
            tokio::task::spawn_blocking(move || {
                let mut running = true;

                println!("hello 4");
                while running {
                    match s.steal() {
                        Steal::Success((path, thumb_tx)) => {
                            if let Some((decoded_image, width_px, height_px)) = resize_image(&path)
                            {
                                let mut output = Box::new(vec![]);
                                let mut encoder = JpegEncoder::new(&mut output);
                                encoder.encode_image(&decoded_image).unwrap();
                                println!("hello 5");

                                let size_bytes = output.len() as u64;
                                let mime_type = "image/jpeg".to_string();

                                let metadata = ImageMetadata {
                                    size_bytes,
                                    mime_type,
                                    width_px,
                                    height_px,
                                };

                                let image = output.into_boxed_slice();
                                println!("hello 6");

                                thumb_tx
                                    .send(PathThumb {
                                        path: path.clone(),
                                        image,
                                        metadata,
                                    })
                                    // .await;
                                    .unwrap();

                                &thumbs_processed.fetch_add(1);

                                debug!("Processing {:.2?}", &time_started_local.elapsed());
                            }
                        }
                        Steal::Empty => {
                            if &thumbs_processed.load() == &total_paths {
                                running = false;
                            }
                            // println!("hello empty {}", &thumbs_processed.load());
                        }
                        Steal::Retry => {
                            println!("retry");
                        }
                    }
                }
            })
        });

        // let mapper_handle = async move {
        //     match worker_receiver.recv().await {
        //         Some(item) => {
        //             subscription_stream.push(item);
        //         }
        //         None => {}
        //     }
        // };
        println!("hello 2");

        let result = /*thumb_rx_stream.scan(
            State::Updated {
                time_started: time_started_local,
            },*/
            thumb_rx_stream.then(|(time_started, total_paths, thumb_rx)| async move {
                println!("hello 3");

                        match thumb_rx.await {
                            //receiver.recv().await {
                            Ok(thumb) => {
                                let time_elapsed = &time_started.elapsed();

                                debug!("Processing {:.2?}", &time_elapsed);

                                    Progress::Updated {
                                        time_elapsed: *time_elapsed,
                                        total_paths,
                                        thumb,
                                    }

                            }
                            Err(_) => // Some(
                                Progress::Finished {
                                    time_elapsed: time_started.elapsed(),
                                    error: None,
                                }
                        }
            },
        );

        tokio::task::spawn(futures::future::join_all(worker_handles));

        Box::pin(result)

        // let sender = Arc::new(sender);
        // let sender = Arc::new(sender);
        // let sender2 = sender.clone();
        // rayon::spawn(move || {
        // let sender = Arc::clone(&sender);
        // let sender2 = sender.clone();

        // paths.for_each(|path| {
        //     if let Some((decoded_image, width_px, height_px)) = resize_image(&path) {
        //         let mut output = Box::new(vec![]);
        //         let mut encoder = JpegEncoder::new(&mut output);
        //         encoder.encode_image(&decoded_image).unwrap();

        //         let size_bytes = output.len() as u64;
        //         let mime_type = "image/jpeg".to_string();

        //         let metadata = ImageMetadata {
        //             size_bytes,
        //             mime_type,
        //             width_px,
        //             height_px,
        //         };

        //         let image = output.into_boxed_slice();

        //         debug!("Processing {:.2?}", &time_started.elapsed());

        //         // let sender = Arc::clone(&sender);

        //         sender
        //             .send(PathThumb {
        //                 path: path.clone(),
        //                 image,
        //                 metadata,
        //             })
        //             .await
        //             .unwrap();
        //         // .unwrap();
        //         // receiver.recv();
        //     }
        // });
        // });

        // let mut iter = receiver.iter();

        // let num_workers = num_cpus::get();

        // let mut thumb_rx = Arc::new(Mutex::new(paths.into_par_stream().map(move |path| {
        //     let time_started = self.time_started.clone();

        //     async move {
        //         if let Some((decoded_image, width_px, height_px)) = resize_image(&path) {
        //             let mut output = Box::new(vec![]);
        //             let mut encoder = JpegEncoder::new(&mut output);
        //             encoder.encode_image(&decoded_image).unwrap();

        //             let size_bytes = output.len() as u64;
        //             let mime_type = "image/jpeg".to_string();

        //             let metadata = ImageMetadata {
        //                 size_bytes,
        //                 mime_type,
        //                 width_px,
        //                 height_px,
        //             };

        //             let image = output.into_boxed_slice();

        //             debug!("Processing {:.2?}", &time_started.elapsed());

        //             Some(PathThumb {
        //                 path: path.clone(),
        //                 image,
        //                 metadata,
        //             })
        //         } else {
        //             None
        //         }
        //     }
        // })));

        // let thumb_rx = Box::pin({
        //     let (map_tx, map_rx) = async_std::channel::unbounded();
        //     let (thumb_tx, thumb_rx) = async_std::channel::unbounded();

        //     let map_fut = async move {
        //         while let Some(path) = path_stream.next().await {
        //             let fut = async_std::task::spawn_blocking(move || {
        //                 if let Some((decoded_image, width_px, height_px)) = resize_image(&path) {
        //                     let mut output = Box::new(vec![]);
        //                     let mut encoder = JpegEncoder::new(&mut output);
        //                     encoder.encode_image(&decoded_image).unwrap();

        //                     let size_bytes = output.len() as u64;
        //                     let mime_type = "image/jpeg".to_string();

        //                     let metadata = ImageMetadata {
        //                         size_bytes,
        //                         mime_type,
        //                         width_px,
        //                         height_px,
        //                     };

        //                     let image = output.into_boxed_slice();

        //                     debug!("Processing {:.2?}", &time_started.elapsed());

        //                     Some(PathThumb {
        //                         path: path.clone(),
        //                         image,
        //                         metadata,
        //                     })
        //                 } else {
        //                     None
        //                 }
        //             });

        //             map_tx.send(fut).await.unwrap();
        //         }
        //     };

        //     let worker_futs: Vec<_> = (0..num_workers)
        //         .map(move |_| {
        //             let map_rx = map_rx.clone();
        //             let thumb_tx = thumb_tx.clone();

        //             let worker_fut = async move {
        //                 while let Ok(fut) = map_rx.recv().await {
        //                     let output = fut.await;
        //                     // println!("issue {:?}", output);

        //                     println!("is closed: {}", thumb_tx.is_closed());
        //                     println!("is empty: {}", thumb_tx.is_empty());
        //                     println!("is full: {}", thumb_tx.is_full());

        //                     match thumb_tx.send(output).await {
        //                         Ok(v) => println!("success {:?}", v),
        //                         Err(e) => println!("halp err {:?}", e),
        //                     };
        //                 }
        //             };
        //             let worker_fut = async_std::task::spawn(worker_fut);
        //             worker_fut
        //         })
        //         .collect();

        //     let par_then_fut =
        //         futures::future::join(map_fut, futures::future::join_all(worker_futs));

        //     async_std::task::spawn(par_then_fut); // ???

        //     thumb_rx
        // });

        // Box::pin(
        //     subscription_stream.scan(State::Updated { time_started }, move |state| {
        //         // let receiver = Arc::clone(&receiver);
        //         // let mut receiver = sender.subscribe();
        //         // move || {
        //         async move {
        //             match state {
        //                 State::Updated {
        //                     time_started,
        //                     // thumb_rx,
        //                     // } => match thumb_rx.lock().await.recv().await {
        //                 } => match receiver.recv().await {
        //                     Some(thumb) => {
        //                         // if let Some(thumb) = next_val {
        //                         debug!("Processing {:.2?}", &time_started.elapsed());

        //                         Some((
        //                             Progress::Updated {
        //                                 time_elapsed: time_started.elapsed(),
        //                                 total_paths: total_paths.clone(),
        //                                 thumb: thumb.clone(),
        //                             },
        //                             State::Updated {
        //                                 time_started,
        //                                 // thumb_rx,
        //                             },
        //                         ))
        //                         // } else {
        //                         //     let error = format!(
        //                         //         "Error decoding image after {:.2?}",
        //                         //         &time_started.elapsed(),
        //                         //     );

        //                         //     Some((
        //                         //         Progress::Finished {
        //                         //             time_elapsed: time_started.elapsed(),
        //                         //             error: Some(error),
        //                         //         },
        //                         //         State::Finished,
        //                         //     ))
        //                         // }
        //                     }
        //                     None => Some((
        //                         Progress::Finished {
        //                             time_elapsed: time_started.elapsed(),
        //                             error: None,
        //                         },
        //                         State::Finished,
        //                     )),
        //                 },
        //                 State::Finished => {
        //                     // We do not let the stream die, as it would start a
        //                     // new download repeatedly if the user is not careful
        //                     // in case of errors.
        //                     // let _: () = iced::futures::future::pending().await;

        //                     None
        //                 }
        //             }
        //         }
        //         // }
        //     }),
        // )
    }
}
