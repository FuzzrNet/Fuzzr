use crate::data::content;
use crate::page;

#[derive(Debug, Clone)]
pub enum Message {
    FileDroppedOnWindow(std::path::PathBuf),
    PageChanged(page::PageType),
    ContentPublished(content::ContentItem),
}
