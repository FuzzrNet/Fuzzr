use iced::{text_input, Element, Row, Text};

use crate::message::Message;

#[derive(Debug, Clone)]
pub struct SettingsPage {
    input_state: text_input::State,
    input_value: String,
}

impl SettingsPage {
    pub fn new() -> SettingsPage {
        SettingsPage {
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
            .push(Text::new("TODO: Settings page").size(16))
            .into()
    }
}
