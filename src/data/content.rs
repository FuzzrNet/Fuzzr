// Application data formats

use libipld::DagCbor;

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
pub struct ImageContent {
    pub buffer: Vec<u8>,
}

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
pub struct PageContent {
    // Metadata(PageMetadata),
    pub content: String,
}

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
pub enum ContentItem {
    Image(ImageContent),
    Page(PageContent),
}

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
pub struct ContentItemBlock {
    content: ContentItem,
}
