// For processing image thumbnails in parallel

// Boilerplate
use iced::Subscription;
use iced_futures::futures;
use log::{debug, error};
use std::time::{Duration, Instant};

// Task dependencies
use crate::data::content::{ImageMetadata, PathThumb};
// use async_std::future;
// use async_std::prelude::*;
// use async_std::task;
use image::codecs::jpeg::JpegEncoder;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};
use par_stream::ParStreamExt;
use std::fs;
use std::hash::Hash;
use std::path::PathBuf;

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

// Utility function
pub fn process_paths(paths: Vec<PathBuf>) -> iced::Subscription<Progress> {
    let start = Instant::now();
    debug!("Processing {} paths", paths.len());
    Subscription::from_recipe(ProcessThumbs { paths, start })
}

fn resize_image(path: &PathBuf) -> Option<(DynamicImage, (u32, u32))> {
    let path = path.clone();
    // task::spawn_blocking(move ||
    match ImageReader::open(&path).unwrap().decode() {
        Ok(img) => Some((img.thumbnail(256, 256), img.dimensions())),
        Err(err) => {
            error!(
                "Error decoding image at path: {:?}\nError was: {}",
                &path, err
            );
            None
        }
    }
    // })
    // .await
}

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

        Box::pin(
            futures::stream::iter(self.paths).par_then_unordered(None, move |path| {
                debug!("Processing {:.2?}", &start.elapsed());

                let image_result = resize_image(&path);

                let result = if let Some((decoded_image, (width_px, height_px))) = image_result {
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
                };

                async move { result }
            }),
        )
    }
}
