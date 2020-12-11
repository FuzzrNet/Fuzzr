use iced::pane_grid;

use crate::data::content;
use crate::page;

#[derive(Debug, Clone)]
pub enum Message {
    FileDroppedOnWindow(std::path::PathBuf),
    PageChanged(page::PageType),
    ContentPublished(content::ContentItem),
    TestButtonPressed,
    Close(pane_grid::Pane),
    SplitPane,
}
