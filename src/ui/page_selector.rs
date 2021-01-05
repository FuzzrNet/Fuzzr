use iced::{button, Button, Column, Element, Row, Text};

use crate::message::Message;
use crate::page::PageType;

#[derive(Debug, Clone)]
pub struct PageButton {
    label_text: String,
    button_state: button::State,
    page_type: PageType,
    is_disabled: bool,
}

#[derive(Debug, Clone)]
pub struct PageSelector {
    buttons: Vec<PageButton>,
    pub active_page: PageType,
}

impl PageSelector {
    pub fn new() -> PageSelector {
        let buttons = vec![
            PageButton {
                label_text: "Publish".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Publish,
                is_disabled: false,
            },
            PageButton {
                label_text: "View".to_string(),
                button_state: button::State::new(),
                page_type: PageType::View,
                is_disabled: false,
            },
            PageButton {
                label_text: "Dashboard".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Dashboard,
                is_disabled: true,
            },
            PageButton {
                label_text: "Feed".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Feed,
                is_disabled: true,
            },
            PageButton {
                label_text: "Site".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Site,
                is_disabled: true,
            },
            PageButton {
                label_text: "Settings".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Settings,
                is_disabled: true,
            },
        ];

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
                row.push(if page_button.is_disabled {
                    Column::new().padding(2).push(
                        Button::new(
                            &mut page_button.button_state,
                            Text::new(page_button.label_text.clone()).size(16),
                        )
                        .style(style::Button::Active {
                            selected: page_button.page_type == *active_page,
                        }),
                    )
                } else {
                    Column::new().padding(2).push(
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
            })
            .into()
    }
}

mod style {
    use iced::{button, Background, Color};

    pub enum Button {
        Active { selected: bool },
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::Active { selected } => {
                    if *selected {
                        button::Style {
                            background: Some(Background::Color(Color::BLACK)),
                            border_color: Color::BLACK,
                            border_radius: 1.0,
                            border_width: 1.0,
                            text_color: Color::WHITE,
                            ..button::Style::default()
                        }
                    } else {
                        button::Style {
                            border_color: Color::BLACK,
                            border_radius: 1.0,
                            border_width: 1.0,
                            text_color: Color::BLACK,
                            ..button::Style::default()
                        }
                    }
                }
            }
        }

        fn hovered(&self) -> button::Style {
            match self {
                Button::Active { selected } => {
                    if *selected {
                        button::Style {
                            background: Some(Background::Color(Color::BLACK)),
                            border_color: Color::BLACK,
                            border_radius: 1.0,
                            border_width: 1.0,
                            text_color: Color::WHITE,
                            ..button::Style::default()
                        }
                    } else {
                        button::Style {
                            background: Some(Background::Color(Color::BLACK)),
                            border_color: Color::BLACK,
                            border_radius: 1.0,
                            border_width: 1.0,
                            text_color: Color::WHITE,
                            ..button::Style::default()
                        }
                    }
                }
            }
        }

        fn disabled(&self) -> button::Style {
            button::Style {
                text_color: Color::BLACK,
                ..button::Style::default()
            }
        }
    }
}
