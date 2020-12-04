use crate::data::content;
use crate::page;

#[derive(Debug, Clone)]
pub enum Message {
    PageChanged(page::PageType),
    ContentPublished(content::ContentItem),
}
