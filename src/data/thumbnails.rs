// For processing image thumbnails in parallel

// Boilerplate
use iced::Subscription;
use iced_futures::futures;
use log::debug;
use std::time::{Duration, Instant};

// Task dependencies
use crate::data::content::PathThumb;
use image::io::Reader as ImageReader;
use par_stream::ParStreamExt;
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

                let result = match ImageReader::open(&path).unwrap().decode() {
                    Ok(img) => {
                        let image = img.thumbnail(256, 256).into_bgra8().to_vec();
                        debug!("Thumbnailed {:.2?}", &start.elapsed());
                        Progress::Finished(PathThumb { path, image }, start.elapsed())
                    }
                    Err(err) => {
                        let error = format!("Error decoding image at {:?}: {}", &path, err);
                        debug!("Errored {:.2?}", &start.elapsed());
                        Progress::Error(error)
                    }
                };

                async move { result }
            }),
        )
    }
}
