use iced::{image, text_input, Column, Container, Element, Image, Length, Text};

use crate::data::content::ContentItem;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct PublishPage {
    input_state: text_input::State,
    input_value: String,
    item: Option<ContentItem>,
    image: Option<image::Handle>,
}

impl PublishPage {
    pub fn new() -> PublishPage {
        PublishPage {
            input_state: text_input::State::new(),
            input_value: String::new(),
            item: None,
            image: None,
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::FileDroppedOnWindow(path) => {
                self.image = Some(image::Handle::from_path(path.clone()));
                self.item = Some(ContentItem {
                    id: 0,
                    path,
                    content: "test path".to_string(),
                });
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<Message> {
        let item_path = match &self.item {
            Some(item) => format!("Path: {:?}", item.path),
            None => "Start adding content by dropping the file or folder here".to_string(),
        };

        let drop_zone = match &self.image {
            Some(image) => Column::new().push(Image::new(image.clone())),
            None => Column::new().push(Text::new(item_path)),
        };

        let publish_container = Column::new().push(drop_zone);

        Container::new(publish_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .into()
    }
}
