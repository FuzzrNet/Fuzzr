use ipfs_embed::core::{Error, Result};
use ipfs_embed::db::StorageService;
use ipfs_embed::net::{NetworkService};
use ipfs_embed::DefaultIpfs;
use ipfs_embed::Ipfs;
use libipld::block::Block;
use libipld::cbor::DagCborCodec;
use libipld::multihash::Code;
use libipld::store::{DefaultParams, Store};
use libipld::{Cid};

use std::fmt;
use std::path::PathBuf;

use async_std::sync::{Arc, Mutex};
use directories_next::ProjectDirs;

use crate::data::content::{ContentItemBlock};

type IpfsEmbed = Ipfs<DefaultParams, StorageService<DefaultParams>, NetworkService<DefaultParams>>;

#[derive(Clone)]
pub struct IpfsClient {
    ipfs: IpfsEmbed,
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

    pub async fn get(&self, cid: &Cid) -> Result<Block<DefaultParams>, Arc<Error>> {
        let content = self.ipfs.get(cid).await?;

        Ok(content)
    }
}

impl fmt::Debug for IpfsClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<TODO IpfsClient debug formatting>")
    }
}

pub type MaybeIpfsClient = Option<Arc<Mutex<IpfsClient>>>;
