use async_std::fs;
use async_std::sync::Arc;
use ipfs_embed::core::{Cid, Error, Result};
use std::path::PathBuf;
use std::str::FromStr;

use crate::data::content::{ContentItem, ContentItemBlock, ImageContent};
use crate::data::ipfs_client::IpfsClientRef;

pub async fn store_file(path: PathBuf, ipfs_client: IpfsClientRef) -> Result<Cid, Arc<Error>> {
    let buffer = fs::read(&path).await.unwrap(); // TODO: error handling
    let block = ContentItemBlock {
        content: ContentItem::Image(ImageContent { buffer }), // TODO: validate via magic number
    };

    let ipfs_client = &ipfs_client.lock().await;
    ipfs_client.add(&block).await
}

pub async fn load_file(
    cid_string: String,
    ipfs_client: IpfsClientRef,
) -> Result<ContentItem, Arc<Error>> {
    let ipfs_client = &ipfs_client.lock().await;
    let cid = Cid::from_str(&cid_string).unwrap();
    let data = ipfs_client.get(&cid).await?;

    Ok(data.content)
}
