// TODO: Not yet ready.

use ipfs_embed::core::{BitswapStorage, BitswapStore, BitswapSync, Error, Result, Storage};
use ipfs_embed::DefaultIpfs;
// use ipfs_embed::core::BitswapStorage;
// use ipfs_embed::db::StorageService;
// use ipfs_embed::net::{NetworkConfig, NetworkService};
// use ipfs_embed::Ipfs;
use libipld::block::Block;
use libipld::cbor::DagCborCodec;
use libipld::multihash::Code;
use libipld::store::{DefaultParams, Store};
use libipld::{alias, Cid, DagCbor};
use std::path::Path;
use std::time::Duration;

use async_std::sync::Arc;

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
struct Identity {
    id: u64,
    name: String,
    age: u8,
}

// bytes: &Vec<u8>

pub async fn add() -> Result<(), Arc<Error>> {
    let path = Path::new("/tmp/local1");
    // let index = sled::open(path.join("index"))?;
    let cache_size: usize = 10;
    let ipfs = DefaultIpfs::default(Some(path.join("blocks")), cache_size).await?;

    // let sweep_interval = Duration::from_millis(10000);
    // let net_config = NetworkConfig::new();
    // let storage = Arc::new(StorageService::open(&sled_config, cache_size, sweep_interval).unwrap());
    // let bitswap_storage = BitswapStorage::new(storage.clone());
    // let network = Arc::new(NetworkService::new(net_config, bitswap_storage).unwrap());
    // let ipfs = Ipfs::new(storage, network);

    // let block = Block::encode(DagCborCodec, Code::Blake3_256, bytes).unwrap();
    let block = Identity {
        id: 0,
        name: "Hunter Trujillo".into(),
        age: 32,
    };
    let ipld_block = libipld::Block::encode(DagCborCodec, Code::Blake3_256, &block)?;
    ipfs.insert(&ipld_block).await?;

    let cid = *ipld_block.cid();

    // let cid = ipfs.insert(&identity).await?;
    let content = ipfs.get(&cid).await?;

    assert_eq!(&ipld_block, &content);
    println!("identity cid is {}", cid);

    Ok(())
}
