use iced::{
    image, scrollable, Align, Column, Command, Container, Element, Image, Length, Row, Scrollable,
    Text,
};
use log::{debug, error, info};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::data::content::PathThumb;
use crate::data::fs_ops::THUMB_SIZE;
use crate::data::thumbnails;
use crate::message::Message;
use crate::ui::style::Theme;

#[derive(Debug, Clone)]
pub struct PublishPage {
    // cid: Option<String>,
    scroll: scrollable::State,
    publish_thumbs: Arc<RwLock<BTreeMap<PathBuf, PathThumb>>>,
    window_width: u16,
}

impl Default for PublishPage {
    fn default() -> Self {
        Self::new()
    }
}

async fn lock_insert(
    publish_thumbs: Arc<RwLock<BTreeMap<PathBuf, PathThumb>>>,
    thumb: PathThumb,
    elapsed: Duration,
    remaining: isize,
) {
    let mut publish_thumbs = publish_thumbs.write().unwrap();
    debug!(
        "Path:{:?}\nImage metadata: {:?}",
        &thumb.path, &thumb.metadata
    );
    publish_thumbs.insert(thumb.path.clone(), thumb);
    info!(
        "thumbnailed {} items after {:.2?}. {} items remaining.",
        publish_thumbs.len(),
        elapsed,
        remaining
    );
}

impl PublishPage {
    pub fn new() -> PublishPage {
        PublishPage {
            scroll: scrollable::State::new(),
            publish_thumbs: Arc::new(RwLock::new(BTreeMap::new())),
            window_width: 800,
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::PathThumbProgress(progress) => match progress {
                thumbnails::Progress::Updated {
                    thumb,
                    start,
                    remaining,
                } => Command::perform(
                    lock_insert(
                        Arc::clone(&self.publish_thumbs),
                        thumb,
                        start.elapsed(),
                        remaining,
                    ),
                    Message::ContentReadyToPublish,
                ),
                thumbnails::Progress::Error { error } => {
                    error!("{}", error);
                    Command::none()
                }
                _ => Command::none(),
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

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        let publish_thumbs = self.publish_thumbs.read().unwrap();

        if publish_thumbs.len() != 0 {
            // Thumbnail column distribution algorithm
            let col_width = Length::Units(THUMB_SIZE as u16);
            let col_count = (self.window_width / (THUMB_SIZE as u16 + 2)) as usize;
            let row_spacing = f32::round(
                (self.window_width as f32 - (col_count as f32 * THUMB_SIZE))
                    / (col_count as f32 - 1.0),
            ) as u16;

            let mut image_grid: Vec<Vec<PathBuf>> = vec![vec![]; col_count];
            let mut heights: Vec<u16> = vec![0; col_count];

            publish_thumbs.iter().for_each(|(path, thumb)| {
                let height_min = heights.iter().min().unwrap();
                let height_index = &heights.iter().position(|h| h == height_min).unwrap();
                image_grid[*height_index].push(path.clone());
                heights[*height_index] += thumb.metadata.height_px as u16;
            });

            let container_cols: Vec<Element<Message>> = image_grid
                .into_iter()
                .map(|image_column| {
                    let col: Element<Message> = Column::with_children(
                        image_column
                            .iter()
                            .filter_map(|path| {
                                if let Some(thumb) = &publish_thumbs.get(path) {
                                    Some(
                                        Image::new(image::Handle::from_pixels(
                                            thumb.metadata.width_px,
                                            thumb.metadata.height_px,
                                            thumb.image.to_vec(),
                                        ))
                                        .into(),
                                    )
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    )
                    .spacing(row_spacing)
                    .width(col_width)
                    .into();
                    let el: Element<Message> = Container::new::<Element<Message>>(col)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into();
                    el
                })
                .collect();

            let row = Row::with_children(container_cols);

            Container::new(
                Scrollable::new(&mut self.scroll)
                    .push(row)
                    .width(Length::Shrink)
                    .align_items(Align::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .style(*theme)
            .into()
        } else {
            Container::new(
                Column::new().push(Text::new("Start adding content by dropping an image here")),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .into()
        }
    }
}
