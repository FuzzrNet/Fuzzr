use std::fmt;
use std::time::Instant;

use ipfs_embed::core::{BitswapStorage, Error, Result, Store, StoreParams};
use ipfs_embed::db::StorageService;
use ipfs_embed::net::{NetworkConfig, NetworkService};
use ipfs_embed::Ipfs;
use libipld::cbor::DagCborCodec;
use libipld::multihash::Code;
use libipld::{Cid, IpldCodec};

use async_std::sync::{Arc, Mutex};
use directories_next::ProjectDirs;
use log::info;

use crate::data::content::ContentItemBlock;
use crate::data::ipfs_bootstrap;

pub type IpfsClientRef = Arc<Mutex<IpfsClient>>;

#[derive(Clone, Debug, Default)]
struct MaxBlockSizeStoreParams;

impl StoreParams for MaxBlockSizeStoreParams {
    const MAX_BLOCK_SIZE: usize = u32::MAX as usize - 1;
    type Codecs = IpldCodec;
    type Hashes = Code;
}

#[derive(Clone)]
pub struct IpfsClient {
    ipfs: Ipfs<
        MaxBlockSizeStoreParams,
        StorageService<MaxBlockSizeStoreParams>,
        NetworkService<MaxBlockSizeStoreParams>,
    >,
    storage: Arc<StorageService<MaxBlockSizeStoreParams>>,
    network: Arc<NetworkService<MaxBlockSizeStoreParams>>,
}

impl IpfsClient {
    pub async fn new() -> Result<IpfsClient, Arc<Error>> {
        let start = Instant::now();

        let sled_config = match ProjectDirs::from("net", "FuzzrNet", "Fuzzr") {
            Some(project_dirs) => {
                sled::Config::new().path(project_dirs.data_local_dir().to_owned())
            }
            None => sled::Config::new().temporary(true),
        };

        let sweep_interval = std::time::Duration::from_millis(10000);
        let cache_size = 1000;

        let storage = Arc::new(StorageService::open(
            &sled_config,
            cache_size,
            Some(sweep_interval),
        )?);

        let bitswap_storage = BitswapStorage::new(storage.clone());

        let bootstrap_list = vec![
            "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
            "/dnsaddr/bootstrap.libp2p.io/p2p/QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
            "/dnsaddr/bootstrap.libp2p.io/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb",
            "/dnsaddr/bootstrap.libp2p.io/p2p/QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
            "/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
            "/ip4/104.131.131.82/udp/4001/quic/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
        ];

        let boot_nodes = ipfs_bootstrap::get_boot_nodes(bootstrap_list);

        info!("Parsed {} bootstrap nodes", boot_nodes.len());

        let net_config = NetworkConfig {
            boot_nodes,
            ..NetworkConfig::new()
        };
        let network = Arc::new(NetworkService::new(net_config, bitswap_storage).await?);

        let ipfs = Ipfs::new(Arc::clone(&storage), Arc::clone(&network));

        info!("IPFS started in {:.2?}", start.elapsed());

        Ok(IpfsClient {
            ipfs,
            storage,
            network,
        })
    }

    pub async fn add(&self, block: &ContentItemBlock) -> Result<Cid, Arc<Error>> {
        let ipld_block = libipld::Block::encode(DagCborCodec, Code::Blake2b256, block)?;
        self.ipfs.insert(&ipld_block).await?;
        let cid = *ipld_block.cid();

        Ok(cid)
    }

    pub async fn get(&self, cid: &Cid) -> Result<ContentItemBlock, Arc<Error>> {
        let block = self.ipfs.get(cid).await?;
        let content_item = block.decode::<DagCborCodec, ContentItemBlock>()?;

        Ok(content_item)
    }
}

impl fmt::Debug for IpfsClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<TODO IpfsClient debug formatting>")
    }
}
