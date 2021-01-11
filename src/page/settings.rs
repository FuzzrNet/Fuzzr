use iced::{text_input, Column, Container, Element, Length, Row, Text, TextInput};

use crate::message::Message;
use crate::ui::style::Theme;

#[derive(Debug, Clone)]
pub struct SettingsPage {
    input_state: text_input::State,
    input_value: String,
    background_input: text_input::State,
    foreground_input: text_input::State,
    background_color: String,
    foreground_color: String,
}

impl SettingsPage {
    pub fn new() -> SettingsPage {
        SettingsPage {
            input_state: text_input::State::new(),
            input_value: String::new(),
            background_input: text_input::State::new(),
            foreground_input: text_input::State::new(),
            background_color: String::new(),
            foreground_color: String::new(),
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::ForegroundChanged(value) => {
                self.input_value = value;
            }
            Message::BackgroundChanged(value) => {
                self.input_value = value;
            }
            _ => {}
        };
    }

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        let settings_container = Column::new()
            .spacing(15)
            .push(Text::new("Customize Theme:").size(16))
            .push(
                Row::new().push(
                    TextInput::new(
                        &mut self.background_input,
                        "Enter Background Color (RGB Hex, i.e, #FF00FF)",
                        &mut self.background_color,
                        Message::BackgroundChanged,
                    )
                    .size(16)
                    .padding(15)
                    .style(*theme)
                    .on_submit(Message::LoadCustomBackground),
                ),
            )
            .push(
                Row::new().push(
                    TextInput::new(
                        &mut self.foreground_input,
                        "Enter Foreground Color (RGB Hex, i.e, #FF00FF)",
                        &mut self.foreground_color,
                        Message::ForegroundChanged,
                    )
                    .size(16)
                    .padding(15)
                    .style(*theme)
                    .on_submit(Message::LoadCustomForeground),
                ),
            );

        Container::new(settings_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .style(*theme)
            .into()
    }
}
