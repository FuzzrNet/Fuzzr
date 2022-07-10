use iced::{
    pure::{column, container, row, text, text_input, Element},
    Command, Length,
};

use crate::message::Message;
use crate::ui::style::Theme;

#[derive(Debug, Clone)]
pub struct SettingsPage {
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
            background_color: String::new(),
            foreground_color: String::new(),
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::ForegroundChanged(value) => {
                self.foreground_color = value;
                Command::none()
            }
            Message::BackgroundChanged(value) => {
                self.background_color = value;
                Command::none()
            }
            _ => Command::none(),
        }
    }

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        let settings_container = row()
            .spacing(15)
            .push(text("Customize Theme:").size(16))
            .push(
                column()
                    .spacing(15)
                    .push(
                        row().push(
                            text_input(
                                "Enter Background Color (RGB Hex, i.e, #00000F)",
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
                        row().push(
                            text_input(
                                "Enter Foreground Color (RGB Hex, i.e, #F100FF)",
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

        container(settings_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .style(*theme)
            .into()
    }
}
