use iced::{text_input, Container, Element, Length, Row, Text};

use crate::message::Message;
use crate::ui::style::Theme;

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

    pub fn update(&mut self, _msg: Message) {}

    pub fn view(&self, theme: &Theme) -> Element<Message> {
        let settings_container = Row::new().push(Text::new("TODO: Settings page").size(16));

        Container::new(settings_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .style(*theme)
            .into()
    }
}
