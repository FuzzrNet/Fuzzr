// For processing image thumbnails in parallel

// Boilerplate
use iced::Subscription;
use iced_futures::futures;
use log::error;
// use std::time::{Duration, Instant};

// Task dependencies
use crate::data::content::PathThumb;
use image::io::Reader as ImageReader;
use par_stream::ParStreamExt;
use std::hash::Hash;
use std::path::PathBuf;

// What is needed to create this task?
pub struct ProcessThumbs {
    tasks: Vec<Progress>,
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
    Finished(Option<PathThumb>),
    Error(String),
}

// Utility function
pub fn process_paths(paths: Vec<PathBuf>) -> iced::Subscription<Progress> {
    let tasks = paths
        .iter()
        .map(|path| Progress::Ready(path.clone()))
        .collect();

    Subscription::from_recipe(ProcessThumbs { tasks })
}

fn process_image_from_path(path: PathBuf) -> Option<PathThumb> {
    match ImageReader::open(&path).unwrap().decode() {
        Ok(img) => {
            // let started = Instant::now();
            let image = img.thumbnail(256, 256).into_bgra8().to_vec();
            // let elapsed = started.elapsed();
            // Some((
            Some(PathThumb { path, image })

            //     State::Progressed,
            // ))
        }
        Err(err) => {
            error!("Error decoding image at {:?}: {}", &path, err);
            None // Swallow error
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
        self.tasks.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(
            futures::stream::iter(self.tasks).par_then_unordered(None, move |task| {
                let mut thumb = None;

                match task {
                    Progress::Ready(path) => {
                        thumb = process_image_from_path(path);
                    }
                    _ => {
                        println!("unexpected");
                    }
                };

                async move { Progress::Finished(thumb) }
            }),
        )
    }
}
