use libipld::Cid;
use std::time::Duration;
use std::path::PathBuf;

type BytesProcessed = u64; // max value: 18.45 exabytes

#[derive(Clone, Debug)]
enum TaskRate {
    ImageScale(Duration, BytesProcessed),
    IpfsAdd(Duration, BytesProcessed),
    IpfsGet(Duration, BytesProcessed),
}

#[derive(Clone, Debug)]
enum TaskState {
    Running,
    Finished(Duration),
    Error(String),
}

#[derive(Clone, Debug)]
pub struct IpfsAddFromFileTask {
    input: PathBuf,
    output: Option<Cid>,
    // state: TaskState,
    // perf: TaskRate,
}

#[derive(Clone, Debug)]
pub struct IpfsGetTask {
    input: Cid,
    output: Option<Vec<u8>>,
    // state: TaskState,
    // perf: TaskRate,
}

#[derive(Clone, Debug)]
pub enum Task {
    // IpfsInit(),
    IpfsAddFromFile(IpfsAddFromFileTask),
    IpfsGet(IpfsGetTask),
    // ImageProcess(),
}
