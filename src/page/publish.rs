use iced::{text_input, Column, Element, Row, Text};

use crate::data::content::ContentItem;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct PublishPage {
    pub input_state: text_input::State,
    pub input_value: String,
    pub item: Option<ContentItem>,
}

impl PublishPage {
    pub fn new() -> PublishPage {
        PublishPage {
            input_state: text_input::State::new(),
            input_value: String::new(),
            item: None,
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::FileDroppedOnWindow(path) => {
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

        let drop_zone = Column::new().push(Text::new(item_path));

        Row::new()
            .push(Text::new("TODO: Publish page").size(16))
            .push(drop_zone)
            .into()
    }
}
