use std::path::PathBuf;
use std::time::Duration;
use std::hash::Hash;

use libipld::Cid;

#[derive(Clone, Debug, Hash)]
pub struct IpfsAddFromFileTask {
    pub input: Option<PathBuf>,
    pub output: Option<Cid>,
    // state: TaskState,
    // perf: TaskRate,
}

#[derive(Clone, Debug, Hash)]
pub struct IpfsGetTask {
    pub input: Option<Cid>,
    pub output: Option<Vec<u8>>,
    // state: TaskState,
    // perf: TaskRate,
}

#[derive(Clone, Debug, Hash)]
pub enum Task {
    // IpfsInit(),
    IpfsAddFromFile(IpfsAddFromFileTask),
    IpfsGet(IpfsGetTask),
    // ImageProcess(),
}

#[derive(Clone, Debug)]
pub enum TasksStatus {
    Working { progress: f32 },
    Idle,
    Errors(Vec<String>),
}

type BytesProcessed = u64; // max value: 18.45 exabytes

#[derive(Clone, Debug)]
pub struct TaskRates {
    pub ipfs_add_from_file: (Duration, BytesProcessed),
    pub ipfs_get: (Duration, BytesProcessed)
}
