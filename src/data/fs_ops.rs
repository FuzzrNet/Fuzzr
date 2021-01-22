use log::{error, info};
use std::path::PathBuf;
use std::time::Instant;

use walkdir::WalkDir;

use image::io::Reader as ImageReader;
use image::GenericImageView;
use rayon::prelude::*;

use crate::data::content::{ImageMetadata, PathThumb};

pub fn walk_dir(path: &PathBuf) -> Vec<PathBuf> {
    let start = Instant::now();
    let mut paths = vec![];

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            paths.push(entry.into_path());
        }
    }

    info!("{} files found in {:.2?}", paths.len(), start.elapsed());

    paths
}

pub const THUMB_SIZE: f32 = 256.0;

pub async fn thumbnail_images(paths: Vec<PathBuf>) -> Vec<PathThumb> {
    let start = Instant::now();

    let results: Vec<PathThumb> = async_std::task::spawn_blocking(|| {
        paths
            .into_par_iter()
            .filter_map(|path| {
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
            })
            .collect()
    })
    .await;

    info!(
        "{} images thumbnailed in {:.2?}",
        &results.len(),
        start.elapsed()
    );

    results
}
