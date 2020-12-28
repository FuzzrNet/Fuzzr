// For storing files in IPFS using a file path, usually used for media (like images), and other networked content

// Boilerplate
use iced_futures::futures;
use log::error;
use std::time::{Duration, Instant};

// Task dependencies
use crate::data::content::{ContentItem, ContentItemBlock, ImageContent};
use crate::data::ipfs_client::MaybeIpfsClient;
use async_std::fs;
use libipld::Cid;
use std::path::PathBuf;

// What is needed to create this task?
pub struct IpfsStoreFile {
    path: PathBuf,
    ipfs_client: MaybeIpfsClient,
}

// What is the result output type?
type Result = Cid;

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
    Stored {
        processed: Size,
        elapsed: Duration,
        result: Result,
    },
    Errored(String),
}

// What steps are required to track task state?
pub enum State {
    Ready(PathBuf, MaybeIpfsClient),
    ReadFileSize {
        path: PathBuf,
        ipfs_client: MaybeIpfsClient,
        started: Instant,
        size: Size,
    },
    LoadedFromFilesystem {
        ipfs_client: MaybeIpfsClient,
        started: Instant,
        size: Size,
        result: ContentItemBlock,
    },
    Finished,
}

// Utility function
pub fn ipfs_store_file_from_path_to_cid(
    path: PathBuf,
    ipfs_client: MaybeIpfsClient,
) -> iced::Subscription<Progress> {
    iced::Subscription::from_recipe(IpfsStoreFile { path, ipfs_client })
}

// Task implementation
impl<H, I> iced_native::subscription::Recipe<H, I> for IpfsStoreFile
where
    H: std::hash::Hasher,
{
    type Output = Progress;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.path.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(
            State::Ready(self.path, self.ipfs_client),
            |state| async move {
                // What needs to happen during each step of the task state? (One step per "await")
                match state {
                    State::Ready(path, ipfs_client) => {
                        let started = Instant::now();

                        match fs::metadata(&path).await {
                            Ok(metadata) => {
                                let size: Size = metadata.len();
                                Some((
                                    Progress::Reading(size),
                                    State::ReadFileSize {
                                        path,
                                        ipfs_client,
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
                                        Progress::Stored {
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
