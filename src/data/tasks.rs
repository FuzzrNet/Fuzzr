use libipld::Cid;
use std::time::Duration;

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
pub struct IpfsAddTask {
    input: Vec<u8>,
    output: Cid,
    state: TaskState,
    perf: TaskRate,
}

// #[derive(Clone, Debug)]
// pub struct IpfsGetTask {
//     input: Cid,
//     output: Vec<u8>,
//     state: TaskState,
//     perf: TaskRate,
// }

#[derive(Clone, Debug)]
pub struct IpfsGetTask {
    input: Cid,
    output: Vec<u8>,
    state: TaskState,
    perf: TaskRate,
}

#[derive(Clone, Debug)]
pub enum Task {
    IpfsInit(),
    IpfsAddTask(IpfsAddTask),
    IpfsGetTask(IpfsGetTask),
    ImageProcessTask(),
}

// #[derive(Clone, Debug)]
// enum TaskType {
//     ImageScale,
//     IpfsAdd,
//     IpfsGet,
// }

impl Task {}
