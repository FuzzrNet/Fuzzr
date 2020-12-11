use iced::{image, text_input, Column, Element, Image, Row, Text};

use crate::message::Message;

#[derive(Debug, Clone)]
pub struct PublishPage {
    input_state: text_input::State,
    input_value: String,
    image: Option<image::Handle>,
}

impl PublishPage {
    pub fn new() -> PublishPage {
        PublishPage {
            input_state: text_input::State::new(),
            input_value: String::new(),
            image: None,
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::FileDroppedOnWindow(path) => {
                self.image = Some(image::Handle::from_path(path.clone()));
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<Message> {
        let drop_zone = match &self.image {
            Some(image) => Column::new().push(Image::new(image.clone())),
            None => Column::new().push(Text::new(
                "Start adding content by dropping the file or folder here",
            )),
        };

        Row::new()
            .push(Text::new("TODO: Publish page").size(16))
            .push(drop_zone)
            .into()
    }
}
