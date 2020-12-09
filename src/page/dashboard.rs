use iced::{
    button, pane_grid, scrollable, Align, Button, Color, Column, Container, Element,
    HorizontalAlignment, Length, PaneGrid, Scrollable, Text,
};

use crate::data::content::ContentItem;
// use crate::data::initialize;
use crate::message::Message;
// use crate::page::PageType;

#[derive(Debug, Clone)]
pub struct DashPage {
    background_color: Color,
    items: Vec<ContentItem>,
    scroll: scrollable::State,
    panes: pane_grid::State<Content>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
    // night_mode: button::State,
}

impl DashPage {
    pub fn new() -> DashPage {
        let (panes, _) = pane_grid::State::new(Content::new(0));

        DashPage {
            background_color: Color::BLACK,
            items: vec![],
            scroll: scrollable::State::new(),
            panes,
            panes_created: 1,
            focus: None,
            // night_mode: button::State::new(),
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            _ => {}
        };
    }

    pub fn view(&mut self) -> Element<Message> {
        let DashPage {
            background_color, ..
        } = self;

        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::new(&mut self.panes, |pane, content| {
            let is_focused = focus == Some(pane);

            let title_bar = pane_grid::TitleBar::new(format!("Pane {}", content.id)).padding(10);
            // .style(style::TitleBar { is_focused });

            pane_grid::Content::new(content.view(pane, total_panes)).title_bar(title_bar)
            // .style(style::Pane { is_focused })
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10);

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}

#[derive(Debug, Clone)]
struct Content {
    id: usize,
    scroll: scrollable::State,
    close: button::State,
}

impl Content {
    fn new(id: usize) -> Self {
        Content {
            id,
            scroll: scrollable::State::new(),
            close: button::State::new(),
        }
    }

    fn view(&mut self, pane: pane_grid::Pane, total_panes: usize) -> Element<Message> {
        let Content { scroll, close, .. } = self;

        let button = |state, label, message, style| {
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

        let mut controls = Column::new()
            .spacing(5)
            .align_items(Align::Start)
            .max_width(150)
            .push(Text::new("Welcome to Fuzzr!!").size(16))
            .push(Text::new("TODO: Relevant user info here").size(14))
            .push(button(
                // Do some button-ey action
                close,
                "Close",
                Message::Close(pane),
                style::Button::Primary,
            ));

        let content = Scrollable::new(scroll)
            .width(Length::Fill)
            .spacing(10)
            .align_items(Align::Center)
            .push(controls);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_y()
            .into()
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
