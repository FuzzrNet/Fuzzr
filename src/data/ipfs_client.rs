#![allow(unused_imports)] // TODO: Dependencies cleanup
use ipfs_embed::core::{BitswapStorage, BitswapStore, BitswapSync, Error, Result, Storage};
use ipfs_embed::db::StorageService;
use ipfs_embed::net::{NetworkConfig, NetworkService};
use ipfs_embed::DefaultIpfs;
use ipfs_embed::Ipfs;
use libipld::block::Block;
use libipld::cbor::DagCborCodec;
use libipld::multihash::Code;
use libipld::store::{DefaultParams, Store};
use libipld::{alias, Cid, DagCbor};

use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use async_std::fs::read;
use async_std::sync::{Arc, Mutex};
use directories_next::ProjectDirs;
// use image::io::Reader as ImageReader;

use crate::data::content::{ContentItem, ImageContent};

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
pub struct ContentItemBlock {
    content: ContentItem,
}

#[derive(Clone)]
pub struct IpfsClient {
    ipfs: Ipfs<DefaultParams, StorageService<DefaultParams>, NetworkService<DefaultParams>>,
}

impl IpfsClient {
    pub async fn new() -> Result<IpfsClient, Arc<Error>> {
        let path = match ProjectDirs::from("net", "FuzzrNet", "Fuzzr") {
            Some(project_dirs) => project_dirs.data_local_dir().to_owned(),
            None => PathBuf::from("/tmp/fuzzr"),
        };
        let cache_size: usize = 10;
        let ipfs = DefaultIpfs::default(Some(path.join("blocks")), cache_size).await?;

        Ok(IpfsClient { ipfs })
    }

    pub async fn add(&self, block: &ContentItemBlock) -> Result<Cid, Arc<Error>> {
        let ipld_block = libipld::Block::encode(DagCborCodec, Code::Blake3_256, block)?;
        self.ipfs.insert(&ipld_block).await?;
        let cid = *ipld_block.cid();

        Ok(cid)
    }

    // TODO: temporary, until task_processor is finished
    pub async fn add_file_from_path(&self, path: PathBuf) -> Result<Cid, Arc<Error>> {
        let buffer = async_std::fs::read(path).await.unwrap();

        let block = ContentItemBlock {
            content: ContentItem::Image(ImageContent { buffer }), // TODO: validate via magic number
        };

        let ipld_block = libipld::Block::encode(DagCborCodec, Code::Blake3_256, &block)?;
        self.ipfs.insert(&ipld_block).await?;
        let cid = *ipld_block.cid();

        Ok(cid)
    }

    pub async fn get(&self, cid: &Cid) -> Result<Block<DefaultParams>, Arc<Error>> {
        let content = self.ipfs.get(cid).await?;

        Ok(content)
    }

    pub async fn ipfs_add_file_from_path(
        ipfs_client: Arc<Mutex<IpfsClient>>,
        path: PathBuf,
    ) -> Result<Cid, Arc<Error>> {
        let ipfs_client = ipfs_client.clone();
        let ipfs_client = ipfs_client.lock().await;
        ipfs_client.add_file_from_path(path).await
    }

    pub async fn ipfs_get(
        ipfs_client: Arc<Mutex<IpfsClient>>,
        cid_string: String,
    ) -> Result<Vec<u8>, Arc<Error>> {
        let ipfs_client = ipfs_client.clone();
        let ipfs_client = ipfs_client.lock().await;
        let cid = Cid::from_str(&cid_string).unwrap();
        let data = ipfs_client.get(&cid).await?;
        Ok(data.to_bytes())
    }
}

impl fmt::Debug for IpfsClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<TODO IpfsClient debug formatting>")
    }
}

// TODO: Graveyard of potentially useful shit
// let index = sled::open(path.join("index"))?;

// let sweep_interval = Duration::from_millis(10000);
// let net_config = NetworkConfig::new();
// let storage = Arc::new(StorageService::open(&sled_config, cache_size, sweep_interval).unwrap());
// let bitswap_storage = BitswapStorage::new(storage.clone());
// let network = Arc::new(NetworkService::new(net_config, bitswap_storage).unwrap());
// let ipfs = Ipfs::new(storage, network);

// let cid = ipfs.insert(&identity).await?;
