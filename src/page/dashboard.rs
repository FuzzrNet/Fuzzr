use iced::{
    button, scrollable, Align, Button, Color, Element, HorizontalAlignment, Length, Row, Text,
};

use crate::data::content::ContentItem;
use crate::message::Message;
// use crate::page::PageType;

#[derive(Debug, Clone)]
pub struct DashPage {
    background_color: Color,
    items: Vec<ContentItem>,
    scroll: scrollable::State,
    night_mode: button::State,
}

impl DashPage {
    pub fn new() -> DashPage {
        DashPage {
            background_color: Color::BLACK,
            items: vec![],
            scroll: scrollable::State::new(),
            night_mode: button::State::new(),
            // buttons: vec![PageButton {
            //     label_text: "TestButton".to_string(),
            //     button_state: button::State::new(),
            //     page_type: PageType::Dashboard,
            // }],
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            _ => {}
        };
    }

    pub fn view(&mut self) -> Element<Message> {
        let DashPage { night_mode, .. } = self;

        // let dash_info_columns = Column::new();
        let test_button = |state, label, message, style| {
            Button::new(
                state,
                Text::new(label)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(16),
            )
            .width(Length::Fill)
            .padding(8)
            .on_press(message)
            .style(style)
        };

        let dash_info_row = Row::new()
            .spacing(20)
            .align_items(Align::Start)
            .push(Text::new("Welcome to Fuzzr!!").size(16))
            .push(Text::new("TODO: Relevant user info here").size(14))
            .push(test_button(
                night_mode,
                "Test Button",
                Message::TestButtonPressed,
                style::Button::Primary,
            ))
            .into();

        dash_info_row
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    // Row,
// Column,
}

mod style {
    use iced::{button, Background, Color, Vector};

    // pub enum Page {
    //     Background,
    // }

    pub enum Button {
        Primary,
        // Secondary,
    }

    // impl container::StyleSheet for Page {
    //     fn style(&self) -> container::Style {
    //         container::Style {
    //             background: Some(Background::Color(match self {
    //                 Page::Background => Color::from_rgb(0.5, 0.5, 0.5),
    //             })),
    //             text_color: Some(Color::BLACK),
    //             border_radius: 3.0,
    //             border_width: 3.0,
    //             border_color: Color::WHITE,
    //             ..container::Style::default()
    //         }
    //     }
    // }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.85, 0.3, 0.1),
                    // Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }
}
