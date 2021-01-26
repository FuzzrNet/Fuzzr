use async_std::sync::Arc;
use ipfs_embed::core::{Cid, Error, Result};

use crate::data::content::{ContentItem, PathThumb};
use crate::data::ipfs_client::IpfsClient;
use crate::data::thumbnails;
use crate::page;
use crate::ui::style::Theme;

use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum Message {
    PageChanged(page::PageType),
    FileDroppedOnWindow(std::path::PathBuf),
    WindowResized { width: u32, height: u32 },
    // IPFS
    IpfsReady(Result<IpfsClient, Arc<Error>>),
    IpfsStoreFile(PathBuf),
    ContentAddedToIpfs(Result<Option<Cid>, Arc<Error>>),
    ContentDroppedOnWindow(Vec<PathBuf>),
    // ContentThumbProcessed(Result<(), Arc<Error>>),
    ContentThumbProcessing(usize),
    ContentReadyToPublish(()),
    PathThumbsProcessed(Vec<PathThumb>),
    PathThumbProgress(thumbnails::Progress),
    ViewPageInputChanged(String),
    ViewPageLoadContent,
    ViewPageContentLoaded(Result<ContentItem, Arc<Error>>),
    SitePageContentChanged(String),
    SitePagePublishButtonClicked,
    // UI / Settings
    BackgroundChanged(String),
    ForegroundChanged(String),
    LoadCustomBackground,
    LoadCustomForeground,
    ThemeChanged(Theme),
}
