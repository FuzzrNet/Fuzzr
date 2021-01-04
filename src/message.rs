use async_std::sync::Arc;
use ipfs_embed::core::{Cid, Error, Result};

use crate::data::content::ContentItem;
use crate::data::ipfs_client::IpfsClient;
use crate::page;

use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum Message {
    PageChanged(page::PageType),
    FileDroppedOnWindow(std::path::PathBuf),
    // IPFS
    IpfsReady(Result<IpfsClient, Arc<Error>>),
    IpfsStoreFile(PathBuf),
    ContentAddedToIpfs(Result<Option<Cid>, Arc<Error>>),
    ViewPageInputChanged(String),
    ViewPageLoadContent,
    ViewPageContentLoaded(Result<ContentItem, Arc<Error>>),
    SitePageContentChanged(String),
    SitePagePublishButtonClicked,
}
