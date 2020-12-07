// Application data formats

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ContentItem {
    pub path: PathBuf,
    pub id: usize,
    pub content: String,
}
