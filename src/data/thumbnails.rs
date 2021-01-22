// For processing image thumbnails in parallel

use async_std::sync::{Arc, Mutex};
use async_std::task::sleep;
use crossbeam_utils::atomic::AtomicCell;
use iced::Subscription;
use iced_futures::futures::{stream, StreamExt};
use image::io::Reader as ImageReader;
use image::GenericImageView;
use log::{debug, error};
use par_stream::{ParMapUnordered, ParStreamExt};
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::data::content::{ImageMetadata, PathThumb};

pub struct ProcessThumbs {
    paths: Arc<Mutex<Vec<PathBuf>>>,
}

#[derive(Debug, Clone)]
pub enum Progress {
    Started {
        start: Instant,
        remaining: isize,
    },
    Updated {
        start: Instant,
        thumb: PathThumb,
        remaining: isize,
    },
    Error {
        error: String,
    },
    Finished,
    Dormant,
    Restarted,
}

enum State {
    Ready {
        start: Instant,
        paths: Arc<Mutex<Vec<PathBuf>>>,
        remaining: Arc<AtomicCell<isize>>,
    },
    Updated {
        start: Instant,
        paths: Arc<Mutex<Vec<PathBuf>>>,
        paths_stream: ParMapUnordered<Progress>,
        remaining: Arc<AtomicCell<isize>>,
    },
    Dormant {
        paths: Arc<Mutex<Vec<PathBuf>>>,
        remaining: Arc<AtomicCell<isize>>,
    },
}

const THUMB_SIZE: f32 = 256.0;

pub fn process_paths(paths: Arc<Mutex<Vec<PathBuf>>>) -> iced::Subscription<Progress> {
    Subscription::from_recipe(ProcessThumbs { paths })
}

fn resize_image(path: &Path) -> Option<PathThumb> {
    let path = path.to_path_buf();

    match ImageReader::open(&path)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
    {
        Ok(img) => {
            let (width_px, height_px) = img.dimensions();
            let aspect_ratio = height_px as f32 / width_px as f32;
            let width_px = THUMB_SIZE;
            let height_px = THUMB_SIZE * aspect_ratio;
            let width_px = width_px as u32;
            let height_px = height_px.round() as u32;

            let thumbnail = img.thumbnail_exact(width_px, height_px);

            let mime_type = "image/jpeg".to_string();
            let image = thumbnail.into_bgra8().into_vec().into_boxed_slice();

            let metadata = ImageMetadata {
                size_bytes: 0, // Thumbnail size doesn't matter because it's not persisted
                mime_type,
                width_px,
                height_px,
            };

            Some(PathThumb {
                path,
                image,
                metadata,
            })
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

impl<H, I> iced_native::subscription::Recipe<H, I> for ProcessThumbs
where
    H: std::hash::Hasher,
{
    type Output = Progress;

    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: stream::BoxStream<'static, I>,
    ) -> stream::BoxStream<'static, Self::Output> {
        stream::unfold(
            State::Dormant {
                paths: self.paths,
                remaining: Arc::new(AtomicCell::new(0)),
            },
            |state| async move {
                match state {
                    State::Ready {
                        start,
                        paths,
                        remaining,
                    } => {
                        let paths_vec = paths.lock().await.to_vec();
                        remaining.fetch_add(paths_vec.len() as isize);
                        let thumbs_remaining = remaining.load();
                        let remaining = Arc::clone(&remaining);
                        let remaining_ref = Arc::clone(&remaining);
                        let paths_stream =
                            stream::iter(paths_vec).par_map_unordered(None, move |path| {
                                let remaining = Arc::clone(&remaining_ref);
                                debug!("Processing {:.2?}, Path: {:?}", &start.elapsed(), &path);
                                move || {
                                    if let Some(thumb) = resize_image(&path) {
                                        remaining.fetch_sub(1);

                                        Progress::Updated {
                                            thumb,
                                            remaining: remaining.load(),
                                            start,
                                        }
                                    } else {
                                        let error = format!(
                                            "Error decoding image after {:.2?}, at: {:?}",
                                            &start.elapsed(),
                                            &path,
                                        );

                                        remaining.fetch_sub(1);

                                        Progress::Error { error }
                                    }
                                }
                            });

                        paths.lock().await.clear();

                        Some((
                            Progress::Started {
                                start,
                                remaining: thumbs_remaining,
                            },
                            State::Updated {
                                start,
                                paths,
                                paths_stream,
                                remaining,
                            },
                        ))
                    }
                    State::Updated {
                        start,
                        paths,
                        mut paths_stream,
                        remaining,
                    } => {
                        if remaining.load() > 0 {
                            if let Some(progress) = paths_stream.next().await {
                                Some((
                                    progress,
                                    State::Updated {
                                        start,
                                        paths,
                                        paths_stream,
                                        remaining,
                                    },
                                ))
                            } else {
                                Some((Progress::Finished, State::Dormant { paths, remaining }))
                            }
                        } else {
                            Some((Progress::Finished, State::Dormant { paths, remaining }))
                        }
                    }
                    State::Dormant { paths, remaining } => {
                        sleep(Duration::from_millis(100)).await;

                        if paths.lock().await.len() > 0 {
                            Some((
                                Progress::Restarted,
                                State::Ready {
                                    paths,
                                    start: Instant::now(),
                                    remaining,
                                },
                            ))
                        } else {
                            Some((Progress::Dormant, State::Dormant { paths, remaining }))
                        }
                    }
                }
            },
        )
        .boxed()
    }
}
