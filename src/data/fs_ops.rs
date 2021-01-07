use anyhow::{Error, Result};
use async_std::sync::Arc;
use log::info;
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

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
