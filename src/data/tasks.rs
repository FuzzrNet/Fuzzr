enum TaskType {
    ImageScale,
    IpfsAdd,
    IpfsGet,
}

type BytesProcessed = u64; // max value: 18.45 exabytes
type Hash = String;

enum TaskRate {
    ImageScale(Duration, BytesProcessed),
    IpfsAdd(Duration, BytesProcessed),
    IpfsGet(Duration, BytesProcessed),
}

enum TaskState {
    Running,
    Finished(Duration),
    Error(String),
}

pub struct IpfsAddTask {
    task_type: IpfsAdd,
    input: Vec<u8>,
    output: Hash,
    state: TaskState,
    perf: TaskRate,
}

pub struct IpfsGetTask {
    task_type: IpfsGet,
    input: Hash,
    output: Vec<u8>,
    state: TaskState,
    perf: TaskRate,
}

pub enum Tasks {
    IpfsAddTask(IpfsAddTask),
    IpfsGetTask(IpfsGetTask),
    ImageProcessTask(),
}
