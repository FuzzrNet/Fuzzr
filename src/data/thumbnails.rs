// For processing image thumbnails in parallel

use async_std::sync::{Arc, Mutex};
use async_std::task::sleep;
use crossbeam_utils::atomic::AtomicCell;
use iced::Subscription;
use iced_futures::futures::{stream, StreamExt};
use image::codecs::jpeg::JpegEncoder;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};
use log::{debug, error};
use par_stream::{ParMapUnordered, ParStreamExt};
use std::hash::Hash;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crate::data::content::{ImageMetadata, PathThumb};

// What is needed to create this task?
pub struct ProcessThumbs {
    paths: Arc<Mutex<Vec<PathBuf>>>,
    // state: Arc<AtomicCell<isize>>,
}

// Size in bytes (max value: 18.45 exabytes)
// type Size = u64;

// #[derive(Debug, Clone)]
// struct Perf {
//     bytes: Size,
//     elapsed: Duration,
// }

// For task output
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
        start: Instant,
        error: String,
        path: PathBuf,
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

// Utility function
pub fn process_paths(paths: Arc<Mutex<Vec<PathBuf>>>) -> iced::Subscription<Progress> {
    Subscription::from_recipe(ProcessThumbs {
        paths,
        // state,
    })
    // debug!("Processing {} paths", paths.len());
    // let state = Arc::new(AtomicCell::new(0));
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
    }

    fn stream(
        self: Box<Self>,
        _input: stream::BoxStream<'static, I>,
    ) -> stream::BoxStream<'static, Self::Output> {
        stream::unfold(
            State::Ready {
                start: Instant::now(),
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
                                    if let Some((decoded_image, width_px, height_px)) =
                                        resize_image(&path)
                                    {
                                        println!("debug 1");
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

                                        remaining.fetch_sub(1);

                                        println!("debug 2");
                                        Progress::Updated {
                                            thumb: PathThumb {
                                                path,
                                                image,
                                                metadata,
                                            },
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

                                        Progress::Error { start, error, path }
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

        //     |(mut paths, start)| async move {
        //         match paths.next().await {
        //             Some(progress) => Some((progress, (paths, start))),
        //             None => {
        //                 let error = format!("Error decoding image after {:.2?}", &start.elapsed(),);
        //                 Some((Progress::Error(error), (paths, start)))
        //             }
        //         }
        //     }, // {
        //        //     Some(p) => Some((paths, p)),
        //        //     None => None,
        //        // }
        // ))
        // .map(|output| {
        //     let state = Arc::clone(&state);
        //     output
        // })
        // .boxed()
    }
}
