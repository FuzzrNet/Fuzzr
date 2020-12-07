// TODO: Not yet ready.

use ipfs_embed::core::BitswapStorage;
use ipfs_embed::db::StorageService;
use ipfs_embed::net::{NetworkConfig, NetworkService};
use ipfs_embed::Ipfs;
use libipld::block::Block;
use libipld::cbor::DagCborCodec;
use libipld::multihash::Code;
use libipld::store::{DefaultParams, Store};
use std::sync::Arc;
use std::time::Duration;

async fn add(bytes: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let sled_config = sled::Config::new().temporary(true);
    let cache_size = 10;
    let sweep_interval = Duration::from_millis(10000);
    let net_config = NetworkConfig::new();
    let storage = Arc::new(StorageService::open(&sled_config, cache_size, sweep_interval).unwrap());
    let bitswap_storage = BitswapStorage::new(storage.clone());
    let network = Arc::new(NetworkService::new(net_config, bitswap_storage).unwrap());
    let ipfs = Ipfs::new(storage, network);

    let block = Block::encode(DagCborCodec, Code::Blake3_256, bytes).unwrap();

    let cid = ipfs.insert(&block).await?;
    let content = ipfs.get(&cid).await?;

    assert_eq!(bytes, &content);
    println!("identity cid is {}", cid);

    Ok(())
}
