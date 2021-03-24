use crossbeam_utils::atomic::AtomicCell;
use iced::{
    image, scrollable, Align, Column, Command, Container, Element, Image, Length, Row, Scrollable,
    Text,
};
use lockfree::map::Map as LockfreeMap;
use log::{debug, error, info};
use std::path::PathBuf;
use std::sync::Arc;
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
    publish_thumbs: Arc<LockfreeMap<PathBuf, PathThumb>>,
    count: Arc<AtomicCell<usize>>,
    window_width: u16,
}

impl Default for PublishPage {
    fn default() -> Self {
        Self::new()
    }
}

async fn insert_thumb(
    publish_thumbs: Arc<LockfreeMap<PathBuf, PathThumb>>,
    count: Arc<AtomicCell<usize>>,
    thumb: PathThumb,
    elapsed: Duration,
    remaining: isize,
) {
    debug!(
        "Path:{:?}\nImage metadata: {:?}",
        &thumb.path, &thumb.metadata
    );
    publish_thumbs.insert(thumb.path.clone(), thumb);
    count.fetch_add(1);
    info!(
        "thumbnailed {} items after {:.2?}. {} items remaining.",
        count.load(),
        elapsed,
        remaining
    );
}

impl PublishPage {
    pub fn new() -> PublishPage {
        PublishPage {
            scroll: scrollable::State::new(),
            publish_thumbs: Arc::new(LockfreeMap::new()),
            count: Default::default(),
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
                    insert_thumb(
                        self.publish_thumbs.clone(),
                        self.count.clone(),
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
        if self.count.load() != 0 {
            // Thumbnail column distribution algorithm
            let col_width = Length::Units(THUMB_SIZE as u16);
            let col_count = (self.window_width / (THUMB_SIZE as u16 + 2)) as usize;
            let row_spacing = f32::round(
                (self.window_width as f32 - (col_count as f32 * THUMB_SIZE))
                    / (col_count as f32 - 1.0),
            ) as u16;

            let mut image_grid: Vec<Vec<PathBuf>> = vec![vec![]; col_count];
            let mut heights: Vec<u16> = vec![0; col_count];

            self.publish_thumbs.iter().for_each(|item| {
                let height_min = heights.iter().min().unwrap();
                let height_index = &heights.iter().position(|h| h == height_min).unwrap();
                image_grid[*height_index].push(item.key().clone());
                heights[*height_index] += item.val().metadata.height_px as u16;
            });

            let container_cols: Vec<Element<Message>> = image_grid
                .into_iter()
                .map(|image_column| {
                    let col: Element<Message> = Column::with_children(
                        image_column
                            .iter()
                            .filter_map(|path| {
                                self.publish_thumbs.get(path).as_ref().map(|thumb| {
                                    Image::new(image::Handle::from_pixels(
                                        thumb.val().metadata.width_px,
                                        thumb.val().metadata.height_px,
                                        thumb.val().image.to_vec(),
                                    ))
                                    .into()
                                })
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
