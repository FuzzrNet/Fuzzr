use iced::{button, Button, Element, Row, Text};

use crate::message::Message;
use crate::page::PageType;

#[derive(Debug, Clone)]
pub struct PageButton {
    label_text: String,
    button_state: button::State,
    page_type: PageType,
    // button_style: ButtonStyle,
}

#[derive(Debug, Clone)]
pub struct PageSelector {
    buttons: Vec<PageButton>,
    pub active_page: PageType,
}

impl PageSelector {
    pub fn new() -> PageSelector {
        let mut buttons = vec![
            PageButton {
                label_text: "Publish".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Publish,
                // button_style: ButtonStyle,
            },
            PageButton {
                label_text: "View".to_string(),
                button_state: button::State::new(),
                page_type: PageType::View,
                // button_style: ButtonStyle,
            },
        ];

        if std::env::var("RUST_LOG").unwrap_or_default() == "fuzzr" {
            let hidden_buttons = vec![
                PageButton {
                    label_text: "Dashboard".to_string(),
                    button_state: button::State::new(),
                    page_type: PageType::Dashboard,
                    // button_style: ButtonStyle,
                },
                PageButton {
                    label_text: "Feed".to_string(),
                    button_state: button::State::new(),
                    page_type: PageType::Feed,
                    // button_style: ButtonStyle,
                },
                PageButton {
                    label_text: "Site".to_string(),
                    button_state: button::State::new(),
                    page_type: PageType::Site,
                    // button_style: ButtonStyle,
                },
                PageButton {
                    label_text: "Settings".to_string(),
                    button_state: button::State::new(),
                    page_type: PageType::Settings,
                    // button_style: ButtonStyle,
                },
                PageButton {
                    label_text: "Testing".to_string(),
                    button_state: button::State::new(),
                    page_type: PageType::Testing,
                    // button_style: ButtonStyle,
                },
            ];

            buttons.extend(hidden_buttons);
        }

        PageSelector {
            buttons,
            active_page: PageType::Publish,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let PageSelector {
            buttons,
            active_page,
        } = self;

        buttons
            .into_iter()
            .fold(Row::new(), |row, page_button| {
                row.push(
                    Button::new(
                        &mut page_button.button_state,
                        Text::new(page_button.label_text.clone()).size(16),
                    )
                    .style(style::Button::Active {
                        selected: page_button.page_type == *active_page,
                    })
                    .on_press(Message::PageChanged(page_button.page_type.clone())),
                )
            })
            .into()
    }
}

// #[derive(Debug, Clone)]
mod style {
    use iced::{button, Background, Color};

    const ACTIVE: Color = Color::from_rgb(
        0xFF as f32 / 255.0,
        0xFF as f32 / 255.0,
        0xFF as f32 / 255.0,
    );

    const HOVERED: Color = Color::from_rgb(
        0x4B as f32 / 255.0,
        0x4B as f32 / 255.0,
        0x4B as f32 / 255.0,
    );

    const INACTIVE: Color = Color::from_rgb(
        0x00 as f32 / 255.0,
        0x00 as f32 / 255.0,
        0x00 as f32 / 255.0,
    );

    pub enum Button {
        Active { selected: bool },
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::Active { selected } => {
                    if *selected {
                        button::Style {
                            background: Some(Background::Color(ACTIVE)),
                            border_radius: 10.0,
                            text_color: Color::WHITE,
                            ..button::Style::default()
                        }
                    } else {
                        button::Style::default()
                    }
                }
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(HOVERED)),
                border_radius: 3.0,
                text_color: Color::BLACK,
                ..self.active()
            }
        }

        fn disabled(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(INACTIVE)),
                border_radius: 3.0,
                text_color: Color::BLACK,
                ..self.hovered()
            }
        }
    }
}
