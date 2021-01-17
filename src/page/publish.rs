use iced::{
    image, scrollable, Align, Column, Command, Container, Element, Image, Length, Row, Scrollable,
    Text,
};

use log::{debug, error, info};
use std::collections::btree_map::BTreeMap;
use std::iter;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::data::content::PathThumb;
use crate::data::thumbnails;
use crate::message::Message;

async fn lock_insert(
    publish_thumbs: Arc<Mutex<Vec<PathThumb>>>,
    thumb: PathThumb,
    elapsed: Duration,
) {
    let mut publish_thumbs = publish_thumbs.lock().unwrap();
    debug!(
        "Path:{:?}\nImage metadata: {:?}",
        &thumb.path, &thumb.metadata
    );
    publish_thumbs.push(thumb);
    info!(
        "thumbnailed {} items after {:.2?}",
        publish_thumbs.len(),
        elapsed
    );
}

#[derive(Debug, Clone)]
pub struct PublishPage {
    // cid: Option<String>,
    scroll: scrollable::State,
    publish_thumbs: Arc<Mutex<Vec<PathThumb>>>,
    // thumb_capacity: usize,
    window_width: u16,
}

impl PublishPage {
    pub fn new() -> PublishPage {
        PublishPage {
            scroll: scrollable::State::new(),
            publish_thumbs: Arc::new(Mutex::new(Vec::new())),
            // thumb_capacity: 0,
            window_width: 800,
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::ContentThumbProgress(progress) => match progress {
                thumbnails::Progress::Updated {
                    time_elapsed,
                    total_paths,
                    thumb,
                } => {
                    // self.thumb_capacity = self.thumb_capacity + thumb.metadata.size_bytes as usize;
                    Command::perform(
                        lock_insert(Arc::clone(&self.publish_thumbs), thumb, time_elapsed),
                        Message::ContentReadyToPublish,
                    )
                }
                thumbnails::Progress::Finished {
                    error,
                    time_elapsed,
                } => {
                    if let Some(error) = error {
                        error!("Error while processing thumbnails: {}", error);
                    } else {
                        let publish_thumbs = self.publish_thumbs.lock().unwrap();
                        info!(
                            "Successfully finished processing {} thumbnails in {:.2?}.",
                            publish_thumbs.len(),
                            time_elapsed
                        )
                    }
                    Command::none()
                }
            },
            Message::WindowResized { width, height: _ } => {
                self.window_width = width as u16;
                Command::none()
            }
            // Message::ContentAddedToIpfs(cid) => match cid {
            //     Ok(cid) => match cid {
            //         Some(cid) => self.cid = Some(cid.to_string()),
            //         None => {}
            //     },
            //     Err(_) => {}
            // },
            _ => Command::none(),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let publish_thumbs = self.publish_thumbs.lock().unwrap();

        if publish_thumbs.len() > 0 {
            // Thumbnail column distribution algorithm
            let col_width = Length::Units(256);
            let col_count = (self.window_width / (256 + 2 + 2)) as usize;

            let mut image_grid: Vec<Vec<usize>> = vec![vec![]; col_count];
            let mut heights: Vec<u16> = vec![0; col_count];

            for (i, thumb) in publish_thumbs.iter().enumerate() {
                let height_min = heights.iter().min().unwrap();
                let height_index = &heights.iter().position(|h| h == height_min).unwrap();
                image_grid[*height_index].push(i);
                heights[*height_index] = heights[*height_index] + thumb.metadata.height_px as u16;
            }

            let container_cols: Vec<Element<Message>> = image_grid
                .into_iter()
                .map(|image_column| {
                    let col: Element<Message> = Column::with_children(
                        image_column
                            .iter()
                            .map(|i| {
                                Image::new(image::Handle::from_memory(
                                    publish_thumbs[*i].image.to_vec(),
                                ))
                                .into()
                            })
                            .collect(),
                    )
                    .width(col_width)
                    .into();
                    let el: Element<Message> = Container::new::<Element<Message>>(col)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        // .style(*theme)
                        .into();
                    el
                })
                .collect();

            let row = Row::with_children(container_cols);

            Container::new(
                Scrollable::new(&mut self.scroll)
                    .push(row)
                    .width(Length::Fill)
                    .align_items(Align::Center)
                    .spacing(2),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
            .into()
        } else {
            Container::new(
                Column::new().push(Text::new("Start adding content by dropping an image here")),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
            .into()
        }
    }
}
