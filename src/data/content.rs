// Application data formats

use libipld::DagCbor;

type Bytes = u64;
type Px = u32;

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
pub struct ImageContent {
    pub buffer: Box<[u8]>,
}

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
pub struct TextContent {
    pub string: String,
}

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
pub struct ImageMetadata {
    pub size_bytes: Bytes,
    pub mime_type: String,
    pub width_px: Px,
    pub height_px: Px,
}

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
pub struct TextMetadata {
    pub size_bytes: Bytes,
}

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
pub enum ContentItem {
    Image(ImageContent, ImageMetadata),
    Text(TextContent, TextMetadata),
}

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
pub struct ContentItemBlock {
    pub content: ContentItem,
}
