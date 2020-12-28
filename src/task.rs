pub mod ipfs_store_file;

use std::path::PathBuf;
use std::time::Duration;
use libipld::Cid;

// #[derive(Clone, Debug)]
// pub enum TasksStatus {
//     Working { progress: f32 },
//     Idle,
//     Errors(Vec<String>),
// }

#[derive(Clone, Debug)]
pub enum Task {
    Idle,
    IpfsStoreFile(PathBuf),
    IpfsLoadCid(Cid),
}

type Size = u64; // max value: 18.45 exabytes

#[derive(Clone, Debug)]
pub struct TaskRates {
    pub ipfs_store_file: (Duration, Size),
    pub ipfs_load_cid: (Duration, Size),
}
