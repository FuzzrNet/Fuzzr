use iced::{button, scrollable, Button, Column, Element, Row, Text};

use crate::data::content::ContentItem;
use crate::message::Message;
use crate::page::PageType;

#[derive(Debug, Clone)]
pub struct PageButton {
    label_text: String,
    button_state: button::State,
    page_type: PageType,
}

#[derive(Debug, Clone)]
pub struct DashPage {
    pub items: Vec<ContentItem>,
    pub buttons: Vec<PageButton>,
    pub scroll: scrollable::State,
}

impl DashPage {
    pub fn new() -> DashPage {
        DashPage {
            items: vec![],
            buttons: vec![PageButton {
                label_text: "TestButton".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Dashboard,
            }],
            scroll: scrollable::State::new(),
        }
    }

    // fn update(&mut self, msg: Message) {
    //   match msg {
    //     Message::ContentPublished(content_item) => {
    //       let DashPage { items, .. } = self;
    //       items.push(content_item);
    //     }
    //     _ => {}
    //   }
    // }

    pub fn view(&self) -> Element<Message, Message> {
        let dash_info_columns = Column::new();
        let dash_info_rows = Row::new();

        let dash_info_rows: Element<_> = dash_info_columns
            .push(Text::new("Welcome to Fuzzr!!").size(18))
            .spacing(20)
            .padding(20)
            .push(Text::new("TODO: Relevant user info here").size(14))
            .spacing(20)
            .padding(20)
            // .push(Button::new(button::State::new(), "TestButton".to_string())
            .into();

        let dash_info_columns: Element<_> = dash_info_columns
            .push(Text::new("Welcome to Fuzzr!!").size(18))
            .spacing(20)
            .padding(20)
            .push(Text::new("TODO: Relevant user info here").size(14))
            .spacing(20)
            .padding(20)
            // .push(Button::new(button::State::new(), "TestButton".to_string())
            .into();

        dash_info_columns,
        dash_info_rows
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
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
