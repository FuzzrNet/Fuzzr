use iced::{scrollable, Element, Row, Text};

use crate::data::content::ContentItem;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct FeedPage {
    pub items: Vec<ContentItem>,
    pub scroll: scrollable::State,
}

impl FeedPage {
    pub fn new() -> FeedPage {
        FeedPage {
            items: vec![],
            scroll: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            _ => {}
        };
    }

    pub fn view(&self) -> Element<Message> {
        Row::new()
            .push(Text::new("TODO: Feed page").size(16))
            .into()
    }
}
