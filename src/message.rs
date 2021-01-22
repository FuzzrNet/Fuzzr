use async_std::sync::Arc;
use ipfs_embed::core::{Cid, Error, Result};

use crate::data::content::ContentItem;
use crate::data::ipfs_client::IpfsClient;
use crate::data::thumbnails;
use crate::page;

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
    ContentThumbProgress(thumbnails::Progress),
    ContentReadyToPublish(()),
    ViewPageInputChanged(String),
    ViewPageLoadContent,
    ViewPageContentLoaded(Result<ContentItem, Arc<Error>>),
    SitePageContentChanged(String),
    SitePagePublishButtonClicked,
}
