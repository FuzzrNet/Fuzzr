use iced::{text_input, Element, Row, Text};

// use crate::data::content::ContentItem;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct TestingPage {
    pub input_state: text_input::State,
    pub input_value: String,
}

impl TestingPage {
    pub fn new() -> TestingPage {
        TestingPage {
            input_state: text_input::State::new(),
            input_value: String::new(),
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            _ => {}
        };
    }

    pub fn view(&self) -> Element<Message> {
        Row::new()
            .push(Text::new("TODO: Testing page").size(16))
            .into()
    }
}
