use iced::{
    image, scrollable, Align, Column, Command, Container, Element, Image, Length, Row, Scrollable,
    Text,
};

use crate::data::content::PathThumb;
use crate::data::fs_ops::THUMB_SIZE;
use crate::message::Message;
use crate::ui::style::Theme;

#[derive(Debug, Clone)]
pub struct PublishPage {
    // cid: Option<String>,
    scroll: scrollable::State,
    publish_thumbs: Vec<PathThumb>,
    thumb_capacity: usize,
    window_width: u16,
}

impl Default for PublishPage {
    fn default() -> Self {
        Self::new()
    }
}

impl PublishPage {
    pub fn new() -> PublishPage {
        PublishPage {
            scroll: scrollable::State::new(),
            publish_thumbs: vec![],
            thumb_capacity: 0,
            window_width: 800,
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::PathThumbsProcessed(thumbs) => {
                self.publish_thumbs = thumbs;
                Command::none()
            }
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
        if !self.publish_thumbs.is_empty() {
            // Thumbnail column distribution algorithm
            let col_width = Length::Units(THUMB_SIZE as u16);
            let col_count = (self.window_width / (THUMB_SIZE as u16 + 2)) as usize;
            let row_spacing = f32::round(
                (self.window_width as f32 - (col_count as f32 * THUMB_SIZE))
                    / (col_count as f32 - 1.0),
            ) as u16;

            println!("row spacing: {}", row_spacing);

            let mut image_grid: Vec<Vec<usize>> = vec![vec![]; col_count];
            let mut heights: Vec<u16> = vec![0; col_count];

            for (i, thumb) in self.publish_thumbs.iter().enumerate() {
                let height_min = heights.iter().min().unwrap();
                let height_index = &heights.iter().position(|h| h == height_min).unwrap();
                image_grid[*height_index].push(i);
                heights[*height_index] += thumb.metadata.height_px as u16;
            }

            let container_cols: Vec<Element<Message>> = image_grid
                .into_iter()
                .map(|image_column| {
                    let col: Element<Message> = Column::with_children(
                        image_column
                            .iter()
                            .map(|i| {
                                Image::new(image::Handle::from_pixels(
                                    self.publish_thumbs[*i].metadata.width_px,
                                    self.publish_thumbs[*i].metadata.height_px,
                                    self.publish_thumbs[*i].image.to_vec(),
                                ))
                                .into()
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
