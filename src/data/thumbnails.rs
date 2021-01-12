// use anyhow::Error;
// use async_std::sync::{Arc, Mutex};
// use image::io::Reader as ImageReader;
// use log::{error, info};
// use rayon::prelude::*;

// use std::collections::btree_map::BTreeMap;
// use std::path::PathBuf;
// // use std::sync::{Arc, Mutex};
// use std::time::Instant;

// use crate::data::content::ContentThumb;

// pub type Thumbs = Arc<BTreeMap<PathBuf, Mutex<ContentThumb>>>;

// pub async fn process_thumbs(thumbs: Thumbs) -> Result<(), Arc<Error>> {
//     let started = Instant::now();

//     // let thumbs = &mut thumbs.lock().await;

//     let thumbs_processed: usize = thumbs
//         .par_iter_mut()
//         .fold(
//             || 0,
//             |count: usize, (path, thumb): (&PathBuf, &Mutex<ContentThumb>)| {
//                 let thumb = thumb.lock().await;

//                 if thumb.thumb.is_none() && thumb.path.is_some() {
//                     if let Some(path) = &thumb.path {
//                         match ImageReader::open(path).unwrap().decode() {
//                             Ok(img) => {
//                                 thumb.thumb = Some(img.thumbnail(256, 256).into_bgra8().to_vec());
//                                 count + 1
//                             }
//                             Err(err) => {
//                                 error!("Error decoding image at {:?}: {}", path, err);
//                                 count
//                             }
//                         }
//                     } else {
//                         count
//                     }
//                 } else {
//                     count
//                 }
//             },
//         )
//         .sum();

//     info!(
//         "Processed {} thumbs in {:.2?}",
//         thumbs_processed,
//         started.elapsed()
//     );

//     Ok(())
// }

// For processing image thumbnails in parallel

// Boilerplate
use iced::Subscription;
use iced_futures::futures;
use log::error;
use std::time::{Duration, Instant};

// Task dependencies
use crate::data::content::PathThumb;
use image::io::Reader as ImageReader;
// use rayon::prelude::*;
use futures::future::FutureExt;
use futures::stream::StreamExt;
use par_stream::ParStreamExt;
use std::hash::Hash;
use std::path::PathBuf;

pub type Paths = Vec<PathBuf>;
type Result = PathThumb;

// What is needed to create this task?
pub struct ProcessThumbs {
    paths: Vec<PathBuf>,
}

// What is the result output type?

// Size in bytes (max value: 18.45 exabytes)
type Size = u64;

// For performance tracking
#[derive(Debug, Clone)]
struct Perf {
    bytes: Size,
    elapsed: Duration,
}

// For task output
#[derive(Debug, Clone, Hash)]
pub enum Progress {
    Ready(PathBuf),
    Finished(PathThumb),
    Error(String),
}
// Processed {
//     // processed: Size,
//     // elapsed: Duration,
// },
// Errored(String),
// Reading(Size),
// Loaded(Duration),
// Processed {
//     processed: Size,
//     elapsed: Duration,
//     result: Result,
// },

// // What steps are required to track task state?
// pub enum State {
//     Ready(Vec<PathBuf>),
//     // Progressed, // { started: Instant, size: Size },
//     Finished(Vec<PathThumb>),
// }

// Utility function
pub fn process_paths(paths: Vec<PathBuf>) -> iced::Subscription<Progress> {
    // let count = thumbs.iter().fold(0, |acc, thumb| {
    //     if thumb.lock().unwrap().thumb.is_none() {
    //         acc + 1
    //     } else {
    //         acc
    //     }
    // });

    // let tasks = paths
    //     .iter()
    //     .map(|path| Progress::Ready(path.clone()))
    //     .collect();

    Subscription::from_recipe(ProcessThumbs { paths })
}

// fn process_image_from_path(path: PathBuf) -> Result<PathThumb> {
//     match ImageReader::open(&path).unwrap().decode() {
//         Ok(img) => {
//             // let started = Instant::now();
//             let thumb = img.thumbnail(256, 256).into_bgra8().to_vec();
//             // let elapsed = started.elapsed();
//             // Some((
//             Ok(PathThumb { path, thumb })

//             //     State::Progressed,
//             // ))
//         }
//         Err(err) => {
//             error!("Error decoding image at {:?}: {}", &path, err);
//             None // Swallow error
//         }
//     }
// }

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
        // let mut stream: futures::stream::Stream<'static, Self::Output> = self
        //     .paths
        //     .into_par_stream()
        Box::pin(
            futures::stream::iter(self.paths).par_then_unordered(None, move |path| {
                // let mut thumbs: Vec<PathThumb> = Vec::new();

                // match lhs {
                //     Progress::Ready(path) => {
                //         if let Some(thumb) = process_image_from_path(path) {
                //             thumbs.push(thumb);
                //         }
                //     }
                //     Progress::Finished(finished_thumbs) => thumbs.extend(finished_thumbs),
                // };

                // match value {
                //     Progress::Ready(path) => {
                //         if let Some(thumb) = process_image_from_path(path) {
                //             thumbs.push(thumb);
                //         }
                //     }
                //     Progress::Finished(finished_thumbs) => thumbs.extend(finished_thumbs),
                // };

                async move {
                    match ImageReader::open(&path).unwrap().decode() {
                        Ok(img) => {
                            // let started = Instant::now();
                            let thumb = img.thumbnail(256, 256).into_bgra8().to_vec();
                            // let elapsed = started.elapsed();
                            // Some((
                            Progress::Finished(PathThumb { path, thumb })
                            //     State::Progressed,
                            // ))
                        }
                        Err(err) => {
                            Progress::Error(format!("Error decoding image at {:?}: {}", &path, err))
                            // None // Swallow error
                        }
                    }
                }
            }),
        )
        // .collect::<Vec<Progress>>();

        // Box::pin(stream.into())
    }
}

// Box::pin(futures::stream::unfold(
//     State::Ready(self.paths),
//     |state| async move {
//         // What needs to happen during each step of the task state? (One step per "await")
//         match state {
//             State::Ready(paths) => {
//                 let started = Instant::now();

//                 paths.par_iter_mut().map(|&mut thumb| {
//                     let locked_thumb = thumb.lock().unwrap();
//                     if locked_thumb.thumb.is_none() && locked_thumb.path.is_some() {
//                         let path = locked_thumb.path.unwrap();
//                         match ImageReader::open(path).unwrap().decode() {
//                             Ok(img) => {
//                                 locked_thumb.thumb =
//                                     Some(img.thumbnail(256, 256).into_bgra8().to_vec());
//                             }
//                             Err(err) => {
//                                 error!("Error decoding image at {:?}: {}", path, err);
//                             }
//                         }
//                     }
//                 });
//             }
//             State::LoadedFromFilesystem {
//                 ipfs_client,
//                 started,
//                 size,
//                 result,
//             } => {
//                 if let Some(ipfs_client) = &ipfs_client {
//                     let ipfs_client = ipfs_client.lock().await;
//                     let result = ipfs_client.add(&result).await;

//                     match result {
//                         Ok(cid) => {
//                             let elapsed = started.elapsed();
//                             Some((
//                                 Progress::Processed {
//                                     processed: size,
//                                     elapsed,
//                                     result: cid,
//                                 },
//                                 State::Finished,
//                             ))
//                         }
//                         Err(err) => {
//                             error!("Could not store file in IPFS: {}", err);
//                             Some((
//                                 Progress::Errored("Could not store file in IPFS".into()),
//                                 State::Finished,
//                             ))
//                         }
//                     }
//                 } else {
//                     error!("Could find IPFS client");
//                     Some((
//                         Progress::Errored("Could not find IPFS client".into()),
//                         State::Finished,
//                     ))
//                 }
//             }
//             State::Finished => {
//                 // Do not change this
//                 let _: () = iced::futures::future::pending().await;

//                 None
//             }
//         }
//     },
// ))
