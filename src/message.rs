use async_std::sync::Arc;
use ipfs_embed::core::{Cid, Error, Result};

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
    IpfsStoreFileProgress(crate::task::ipfs_store_file::Progress),
    // TODO: OLD:
    ContentAddedToIpfs(Result<Cid, Arc<Error>>),
    ContentPageInputChanged(String),
    ContentPageLoadContent,
    ContentPageImageLoaded(Result<Vec<u8>, Arc<Error>>),
    SitePageContentChanged(String),
    SitePagePublishButtonClicked,
}
