use std::hash::Hash;

use iced::futures;
use iced::Subscription;

use crate::data::ipfs_client::IpfsClient;
use crate::data::tasks::Task;

// #[derive(Clone, Debug)]
// enum TaskInput {}

// #[derive(Clone, Debug)]
// struct Task {
//     r#type: TaskType,
//     input: TaskInput,
//     output: TaskData,
//     state: TaskState,
// }

#[derive(Debug, Clone)]
pub enum Progress {
    Started,
    Advanced(f32),
    Finished,
    Errored,
}

pub enum State {
    Ready(String),
    Downloading {
        response: reqwest::Response,
        total: u64,
        downloaded: u64,
    },
    Finished,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct TaskProcessor {
    ipfs_client: Option<IpfsClient>,
    tasks: Vec<Task>,
    threads: usize,
}

// impl TaskProcessor {
//     pub fn new(threads: usize) -> TaskProcessor {
//         TaskProcessor {
//             ipfs_client: None,
//             tasks: vec![],
//             threads,
//         }
//     }
// }

impl<H, I> iced_native::subscription::Recipe<H, I> for TaskProcessor
where
    H: std::hash::Hasher,
{
    type Output = Progress;

    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        self.tasks.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(
            State::Ready(self.url),
            |state| async move {
                match state {
                    State::Ready(url) => {
                        let response = reqwest::get(&url).await;

                        match response {
                            Ok(response) => {
                                if let Some(total) = response.content_length() {
                                    Some((
                                        Progress::Started,
                                        State::Downloading {
                                            response,
                                            total,
                                            downloaded: 0,
                                        },
                                    ))
                                } else {
                                    Some((Progress::Errored, State::Finished))
                                }
                            }
                            Err(_) => Some((Progress::Errored, State::Finished)),
                        }
                    }
                    State::Downloading {
                        mut response,
                        total,
                        downloaded,
                    } => match response.chunk().await {
                        Ok(Some(chunk)) => {
                            let downloaded = downloaded + chunk.len() as u64;

                            let percentage = (downloaded as f32 / total as f32) * 100.0;

                            Some((
                                Progress::Advanced(percentage),
                                State::Downloading {
                                    response,
                                    total,
                                    downloaded,
                                },
                            ))
                        }
                        Ok(None) => Some((Progress::Finished, State::Finished)),
                        Err(_) => Some((Progress::Errored, State::Finished)),
                    },
                    State::Finished => {
                        // We do not let the stream die, as it would start a
                        // new download repeatedly if the user is not careful
                        // in case of errors.
                        let _: () = iced::futures::future::pending().await;

                        None
                    }
                }
            },
        ))
    }
}

pub fn task_processor<T: ToString>(url: T) -> Subscription<Progress> {
    Subscription::from_recipe(TaskProcessor {
        url: url.to_string(),
    })
}

/*
match client.add(data).await {
    Ok(res) => println!("{}", res.hash),
    Err(e) => eprintln!("error adding file: {}", e),
}
*/
