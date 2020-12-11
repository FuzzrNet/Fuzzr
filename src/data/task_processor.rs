use ipfs_api::IpfsClient;

enum TaskInput {}

struct Task {
    r#type: TaskType,
    input: TaskInput,
    output: TaskData,
    state: TaskState,
}

pub struct TaskProcessor {
    ipfs_client: Option<IpfsClient>,
    tasks: Vec<Task>,
    threads: usize,
}

impl TaskProcessor {
    pub fn new(threads: usize) -> TaskProcessor {
        TaskProcessor {
            ipfs_client: None,
            tasks: vec![],
            threads,
        }
    }
}

/*
match client.add(data).await {
    Ok(res) => println!("{}", res.hash),
    Err(e) => eprintln!("error adding file: {}", e),
}
*/
