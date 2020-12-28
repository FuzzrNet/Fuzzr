use std::hash::Hash;
use std::time::Duration;

use iced_futures::futures;

use crate::data::tasks::Task;

// Just a little utility function
pub fn run_task<T: ToString>(task: Task) -> iced::Subscription<Progress> {
    iced::Subscription::from_recipe(TaskProcessor {
        task,
    })
}

#[derive(Hash)]
pub struct TaskProcessor {
    task: Task,
}

impl<H, I> iced_native::subscription::Recipe<H, I> for TaskProcessor
where
    H: std::hash::Hasher,
{
    type Output = Progress;

    fn hash(&self, state: &mut H) {

        std::any::TypeId::of::<Self>().hash(state);
        self.task.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(
            State::Ready(self.task),
            |task_state| async move {
                match task_state {
                    Task::IpfsAddFromFile(task) => {
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

#[derive(Debug, Clone)]
pub enum Progress {
    Started,
    Working(f32),
    Finished,
    Errored,
}

// pub enum State {
//     Ready(String),
//     Downloading {
//         response: reqwest::Response,
//         total: u64,
//         downloaded: u64,
//     },
//     Finished,
// }

type BytesProcessed = u64; // max value: 18.45 exabytes

#[derive(Clone, Debug)]
pub enum State {
    Ready(Task),
    Finished(Duration, BytesProcessed),
    Error(String),
}
