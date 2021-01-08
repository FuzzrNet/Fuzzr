// For storing files in IPFS using a file path, usually used for media (like images), and other networked content

// Boilerplate
use iced::Subscription;
use iced_futures::futures;
use log::error;
use std::time::{Duration, Instant};

// Task dependencies
use crate::data::content::{ContentThumb, ImageContent};
use async_std::fs;
use image::io::Reader as ImageReader;
use rayon::prelude::*;
use std::hash::Hash;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub type Thumbs = Arc<Vec<Mutex<ContentThumb>>>;

// What is needed to create this task?
pub struct ProcessThumbs {
    thumbs: Thumbs,
    count: usize,
}

// What is the result output type?
type Result = Thumbs;

// Size in bytes (max value: 18.45 exabytes)
type Size = u64;

// For performance tracking
#[derive(Debug, Clone)]
struct Perf {
    bytes: Size,
    elapsed: Duration,
}

// For task output
#[derive(Debug, Clone)]
pub enum Progress {
    Started,
    Reading(Size),
    Loaded(Duration),
    Processed {
        processed: Size,
        elapsed: Duration,
        result: Result,
    },
    Errored(String),
}

// What steps are required to track task state?
pub enum State {
    Ready(Thumbs),
    ReadFileSize {
        thumbs: Thumbs,
        started: Instant,
        size: Size,
    },
    LoadedFromFilesystem {
        started: Instant,
        size: Size,
        result: ContentThumb,
    },
    Finished,
}

// Utility function
pub fn process_thumbs(thumbs: Thumbs) -> iced::Subscription<Progress> {
    let count = thumbs.iter().fold(0, |acc, thumb| {
        if thumb.lock().unwrap().thumb.is_none() {
            acc += 1;
        }
        acc
    });

    Subscription::from_recipe(ProcessThumbs { thumbs, count })
}

// Task implementation
impl<H, I> iced_native::subscription::Recipe<H, I> for ProcessThumbs
where
    H: std::hash::Hasher,
{
    type Output = Progress;

    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        self.count.hash(state);
    }

    fn stream(
        self: Box<Self>,
        input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(
            State::Ready(self.thumbs),
            |state| async move {
                // What needs to happen during each step of the task state? (One step per "await")
                match state {
                    State::Ready(thumbs) => {
                        let started = Instant::now();

                        let img_path = thumbs.par_iter_mut().map(|&mut thumb| {
                            let locked_thumb = thumb.lock().unwrap();
                            if locked_thumb.thumb.is_none() && locked_thumb.path.is_some() {
                                let img = ImageReader::open(locked_thumb.path.unwrap())
                                    .unwrap()
                                    .decode()
                                    .unwrap();

                                locked_thumb.thumb =
                                    Some(Box::new(img.thumbnail(256, 256).into_bgra8().as_bytes()));
                            }
                        });

                        match fs::metadata(&thumbs).await {
                            Ok(metadata) => {
                                let size: Size = metadata.len();
                                Some((
                                    Progress::Reading(size),
                                    State::ReadFileSize {
                                        thumbs,
                                        started,
                                        size,
                                    },
                                ))
                            }
                            Err(err) => {
                                error!("Could not read file size: {}", err);
                                Some((
                                    Progress::Errored("Could not read file size".into()),
                                    State::Finished,
                                ))
                            }
                        }
                    }
                    State::ReadFileSize {
                        path,
                        ipfs_client,
                        started,
                        size,
                    } => {
                        match fs::read(&path).await {
                            Ok(result) => {
                                let elapsed = started.elapsed();
                                let block = ContentItemBlock {
                                    content: ContentItem::Image(ImageContent { buffer: result }), // TODO: validate via magic number
                                };
                                Some((
                                    Progress::Loaded(elapsed),
                                    State::LoadedFromFilesystem {
                                        ipfs_client,
                                        started,
                                        size,
                                        result: block,
                                    },
                                ))
                            }
                            Err(err) => {
                                error!("Could not read file data: {}", err);
                                Some((
                                    Progress::Errored("Could not read file data".into()),
                                    State::Finished,
                                ))
                            }
                        }
                    }
                    State::LoadedFromFilesystem {
                        ipfs_client,
                        started,
                        size,
                        result,
                    } => {
                        if let Some(ipfs_client) = &ipfs_client {
                            let ipfs_client = ipfs_client.lock().await;
                            let result = ipfs_client.add(&result).await;

                            match result {
                                Ok(cid) => {
                                    let elapsed = started.elapsed();
                                    Some((
                                        Progress::Processed {
                                            processed: size,
                                            elapsed,
                                            result: cid,
                                        },
                                        State::Finished,
                                    ))
                                }
                                Err(err) => {
                                    error!("Could not store file in IPFS: {}", err);
                                    Some((
                                        Progress::Errored("Could not store file in IPFS".into()),
                                        State::Finished,
                                    ))
                                }
                            }
                        } else {
                            error!("Could find IPFS client");
                            Some((
                                Progress::Errored("Could not find IPFS client".into()),
                                State::Finished,
                            ))
                        }
                    }
                    State::Finished => {
                        // Do not change this
                        let _: () = iced::futures::future::pending().await;

                        None
                    }
                }
            },
        ))
    }
}
