use crate::data::content;
use crate::page;

use async_std::sync::Arc;
use ipfs_embed::core::Error;
use ipfs_embed::core::Result;

#[derive(Debug, Clone)]
pub enum Message {
    FileDroppedOnWindow(std::path::PathBuf),
    PageChanged(page::PageType),
    ContentPublished(content::ContentItem),
    ContentSuccess(Result<(), Arc<Error>>),
}
