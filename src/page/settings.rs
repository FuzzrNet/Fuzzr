use iced::{text_input, Color, Column, Command, Container, Element, Length, Row, Text, TextInput};

use crate::message::Message;
use crate::ui::style::Theme;

#[derive(Debug, Clone)]
pub struct SettingsPage {
    background_input: text_input::State,
    foreground_input: text_input::State,
    background_color: String,
    foreground_color: String,
}

impl Default for SettingsPage {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsPage {
    pub fn new() -> SettingsPage {
        SettingsPage {
            background_input: text_input::State::new(),
            foreground_input: text_input::State::new(),
            foreground_color: String::new(),
            background_color: String::new(),
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            // Message::BackgroundInputChanged(value) => {
            //     self.background_input = value;
            //     Command::none()
            // }
            // Message::ForegroundInputChanged(value) => {
            //     self.foreground_input = value;
            //     Command::none()
            // }
            Message::BackgroundChanged(value) => {
                convert_to_rgb(self.background_color.clone().as_str());
                self.background_color = value;
                // println!("{:?}", value);
                Command::none()
            }
            Message::ForegroundChanged(value) => {
                self.foreground_color = value;
                Command::none()
            }
            _ => Command::none(),
        }
    }

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        let settings_container = Row::new()
            .spacing(15)
            .push(Text::new("Customize Theme:").size(16))
            .push(
                Column::new()
                    .spacing(15)
                    .push(
                        Row::new().push(
                            TextInput::new(
                                &mut self.background_input,
                                "Enter Background Color (RGB Hex, i.e, #FF00FF)",
                                &self.background_color,
                                Message::BackgroundChanged,
                            )
                            .size(16)
                            .width(Length::Units(450))
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
                                &self.foreground_color,
                                Message::ForegroundChanged,
                            )
                            .size(16)
                            .width(Length::Units(450))
                            .padding(15)
                            .style(*theme)
                            .on_submit(Message::LoadCustomForeground),
                        ),
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

// let hex = "#A84D00";
// convert_to_rgb(hex);

pub fn convert_to_rgb(hex: &str) -> Color {
    let red = u16::from_str_radix(&hex[0..2], 16).unwrap() as f32;
    let green = u16::from_str_radix(&hex[2..4], 16).unwrap() as f32;
    let blue = u16::from_str_radix(&hex[4..6], 16).unwrap() as f32;

    let new_color = Color::new(red, green, blue, 1.0);
    new_color

    // let red = red as f32;
    // let green = green as f32;
    // let blue = blue as f32;

    // println!("r {:?} , g {:?} , b {:?}", red, green, blue);
    // .parse::<u16>()?
    // .parse::<u16>()?
    // .parse::<u16>()?

    // println!("{:?}", new_color);
}
