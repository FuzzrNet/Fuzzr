use async_std::fs;
use async_std::sync::Arc;
use ipfs_embed::core::{Cid, Error, Result};
use log::error;
use std::path::PathBuf;
use std::str::FromStr;

use crate::data::content::{
    ContentItem, ContentItemBlock, ImageContent, ImageMetadata, TextContent, TextMetadata,
};
use crate::data::ipfs_client::IpfsClientRef;

pub async fn store_file(
    path: PathBuf,
    ipfs_client: IpfsClientRef,
) -> Result<Option<Cid>, Arc<Error>> {
    let file_metadata = fs::metadata(&path).await.unwrap();
    let size_bytes = file_metadata.len();
    let buffer = fs::read(&path).await.unwrap(); // TODO: error handling

    if infer::is_image(&buffer[0..4]) {
        let (width_px, height_px) = image::image_dimensions(path).unwrap();
        let mime_type = infer::get(&buffer[0..4]).unwrap().mime_type().to_string();

        let block = ContentItemBlock {
            content: ContentItem::Image(
                ImageContent { buffer },
                ImageMetadata {
                    size_bytes,
                    width_px,
                    height_px,
                    mime_type,
                },
            ),
        };

        let ipfs_client = &ipfs_client.lock().await;
        let cid = ipfs_client.add(&block).await?;

        Ok(Some(cid))
    } else {
        match String::from_utf8(buffer) {
            Ok(string) => {
                let block = ContentItemBlock {
                    content: ContentItem::Text(TextContent { string }, TextMetadata { size_bytes }),
                };

                let ipfs_client = &ipfs_client.lock().await;
                let cid = ipfs_client.add(&block).await?;

                Ok(Some(cid))
            }
            Err(err) => {
                error!(
                    "Error decoding file as text (probably an unhandled binary file): {}",
                    err
                );
                Ok(None)
            }
        }
    }
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
