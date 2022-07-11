use std::fmt;

use ipfs_embed::{identity::ed25519::Keypair, Block, Config, DefaultParams, Ipfs};
use libipld::{cbor::DagCborCodec, multihash::Code, store::StoreParams, Cid, IpldCodec, Result};

use anyhow::Error;
use async_std::sync::{Arc, RwLock};
use directories_next::ProjectDirs;

use crate::data::content::ContentItemBlock;

pub type IpfsClientRef = Arc<RwLock<IpfsClient>>;

#[derive(Clone, Debug, Default)]
struct MaxBlockSizeStoreParams;

// TODO: refer to https://github.com/ipfs-rust/ipfs-embed/blob/87ac5acc1e8c8a0dad9e74560fdfea22e02ba98d/examples/compat.rs#L8-L12 for future IPFS compat
impl StoreParams for MaxBlockSizeStoreParams {
    const MAX_BLOCK_SIZE: usize = u32::MAX as usize - 1;
    type Codecs = IpldCodec;
    type Hashes = Code;
}

#[derive(Clone)]
pub struct IpfsClient {
    ipfs: Ipfs<DefaultParams>,
}

impl IpfsClient {
    pub async fn new() -> Result<IpfsClient, Arc<Error>> {
        let config = match ProjectDirs::from("net", "FuzzrNet", "Fuzzr") {
            Some(project_dirs) => Config::new(
                project_dirs.data_local_dir().join("sqlite").as_path(),
                Keypair::generate(), // TODO: persist keypair
            ),
            None => Config::default(),
        };

        let ipfs = Ipfs::new(config).await?;

        Ok(IpfsClient { ipfs })
    }

    pub async fn add(&self, block: &ContentItemBlock) -> Result<Cid, Arc<Error>> {
        let ipld_block = Block::encode(DagCborCodec, Code::Blake2b256, block)?;
        let cid = *ipld_block.cid();
        self.ipfs.insert(ipld_block)?;

        Ok(cid)
    }

    pub fn get(&self, cid: &Cid) -> Result<ContentItemBlock, Arc<Error>> {
        let block = self.ipfs.get(cid)?;
        let content_item = block.decode::<DagCborCodec, ContentItemBlock>()?;

        Ok(content_item)
    }
}

impl fmt::Debug for IpfsClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<TODO IpfsClient debug formatting>")
    }
}
