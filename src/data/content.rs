// Application data formats

use libipld::{Cid, DagCbor};
use std::{hash::Hash, path::PathBuf};

type Bytes = u64;
type Px = u32;

#[derive(Clone, DagCbor, Debug, Eq, PartialEq, Hash)]
pub struct ImageContent {
    pub buffer: Box<[u8]>,
}

#[derive(Clone, DagCbor, Debug, Eq, PartialEq, Hash)]
pub struct TextContent {
    pub string: String,
}

#[derive(Clone, DagCbor, Debug, Eq, PartialEq, Hash)]
pub struct ImageMetadata {
    pub size_bytes: Bytes,
    pub mime_type: String,
    pub width_px: Px,
    pub height_px: Px,
}

#[derive(Clone, DagCbor, Debug, Eq, PartialEq, Hash)]
pub struct TextMetadata {
    pub size_bytes: Bytes,
}

#[derive(Clone, DagCbor, Debug, Eq, PartialEq, Hash)]
pub enum ContentItem {
    Image(ImageContent, ImageMetadata),
    Text(TextContent, TextMetadata),
}

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
pub struct ContentItemBlock {
    pub content: ContentItem,
    pub size_bytes: Bytes, // Content size, not total block size
}

#[derive(Clone, Debug, Hash)]
pub struct ContentThumb {
    pub cid: Cid,
    pub image: Vec<u8>,
}

#[derive(Clone, Debug, Hash)]
pub struct PathThumb {
    pub path: PathBuf,
    pub image: Box<[u8]>,
    pub metadata: ImageMetadata,
}
