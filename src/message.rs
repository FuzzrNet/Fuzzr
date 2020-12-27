use iced::pane_grid;

use crate::page;

#[derive(Clone, Debug)]
pub enum Message {
    FileDroppedOnWindow(std::path::PathBuf),
    PageChanged(page::PageType),
    TestButtonPressed,
    Close(pane_grid::Pane),
    SplitPane,
    ContentPageInputChanged(String),
    ContentPageLoadContent,
    #[cfg(feature = "ipfs_rs")]
    Backend(ipfs::IpfsMessage)
    #[cfg(feature = "sled_db")]
    Backend(sled::SledMessage)
}

#[cfg(feature = "ipfs_rs")]
pub mod ipfs {
    use async_std::sync::Arc;
    use ipfs_embed::core::{Cid, Error, Result};

    use crate::data::content;
    use crate::data::ipfs_client::IpfsClient;

    #[derive(Clone, Debug)]
    pub enum IpfsMessage {
        ContentPublished(content::ContentItem),
        IpfsReady(Result<IpfsClient, Arc<Error>>),
        ContentAddedToIpfs(Result<Cid, Arc<Error>>),
        ContentPageImageLoaded(Result<Vec<u8>, Arc<Error>>),
    }
}

#[cfg(feature = "sled_db")]
pub mod sled {
    use async_std::sync::Arc;
    use ipfs_embed::core::{Cid, Error, Result};

    use crate::data::content;
    use crate::data::ipfs_client::IpfsClient;

    #[derive(Clone, Debug)]
    pub enum IpfsMessage {
        ContentPublished(content::ContentItem),
        IpfsReady(Result<IpfsClient, Arc<Error>>),
        ContentAddedToIpfs(Result<Cid, Arc<Error>>),
        ContentPageImageLoaded(Result<Vec<u8>, Arc<Error>>),
    }
}
