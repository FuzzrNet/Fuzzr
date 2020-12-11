use iced_futures::futures;

// TODO initialize ipfs, xmrig
// https://github.com/hecrj/iced/blob/master/examples/download_progress/src/download.rs

// pub fn file<T: ToString>(url: T) -> iced::Subscription<Progress> {
//     iced::Subscription::from_recipe(Initialize {
//         url: url.to_string(),
//     })
// }

pub struct Initialize {
    url: String,
}

// Make sure iced can use our download stream
impl<H, I> iced_native::subscription::Recipe<H, I> for Initialize
where
    H: std::hash::Hasher,
{
    type Output = Progress;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.url.hash(state);
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
                                        State::Initializing {
                                            response,
                                            total,
                                            initialized: 0,
                                        },
                                    ))
                                } else {
                                    Some((Progress::Errored, State::Finished))
                                }
                            }
                            Err(_) => Some((Progress::Errored, State::Finished)),
                        }
                    }
                    State::Initializing {
                        mut response,
                        total,
                        initialized,
                    } => match response.chunk().await {
                        Ok(Some(chunk)) => {
                            let initialized = initialized + chunk.len() as u64;

                            let percentage = (initialized as f32 / total as f32) * 100.0;

                            Some((
                                Progress::Advanced(percentage),
                                State::Initializing {
                                    response,
                                    total,
                                    initialized,
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
    Advanced(f32),
    Finished,
    Errored,
}

pub enum State {
    Ready(String),
    Initializing {
        response: reqwest::Response,
        total: u64,
        initialized: u64,
    },
    Finished,
}
