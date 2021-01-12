// For processing image thumbnails in parallel

// Boilerplate
use iced::Subscription;
use iced_futures::futures;
// use std::time::{Duration, Instant};

// Task dependencies
use crate::data::content::PathThumb;
use image::io::Reader as ImageReader;
use par_stream::ParStreamExt;
use std::hash::Hash;
use std::path::PathBuf;

// What is needed to create this task?
pub struct ProcessThumbs {
    paths: Vec<PathBuf>,
}

// What is the result output type?

// Size in bytes (max value: 18.45 exabytes)
// type Size = u64;

// TODO: For performance tracking
// #[derive(Debug, Clone)]
// struct Perf {
//     bytes: Size,
//     elapsed: Duration,
// }

// For task output
#[derive(Debug, Clone, Hash)]
pub enum Progress {
    Ready(PathBuf),
    Finished(PathThumb),
    Error(String),
}

// Utility function
pub fn process_paths(paths: Vec<PathBuf>) -> iced::Subscription<Progress> {
    Subscription::from_recipe(ProcessThumbs { paths })
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
        Box::pin(
            futures::stream::iter(self.paths).par_then_unordered(None, move |path| {
                let result = match ImageReader::open(&path).unwrap().decode() {
                    Ok(img) => {
                        let image = img.thumbnail(256, 256).into_bgra8().to_vec();
                        Progress::Finished(PathThumb { path, image })
                    }
                    Err(err) => {
                        let error = format!("Error decoding image at {:?}: {}", &path, err);
                        Progress::Error(error)
                    }
                };

                async move { result }
            }),
        )
    }
}
