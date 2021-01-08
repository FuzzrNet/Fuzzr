use async_std::sync::Arc;
use ipfs_embed::core::{Cid, Error, Result};

use crate::data::content::{ContentItem, ContentThumb};
use crate::data::ipfs_client::IpfsClient;
use crate::data::process_thumbs;
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
    ContentDroppedOnWindow(Vec<PathBuf>),
    ContentThumbProgress(process_thumbs::Progress),
    ViewPageInputChanged(String),
    ViewPageLoadContent,
    ViewPageContentLoaded(Result<ContentItem, Arc<Error>>),
    SitePageContentChanged(String),
    SitePagePublishButtonClicked,
}
