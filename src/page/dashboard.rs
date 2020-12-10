use iced::{
    button, pane_grid, scrollable, Align, Button, Column, Container, Element, HorizontalAlignment,
    Length, PaneGrid, Scrollable, Text,
};

use crate::data::content::ContentItem;
// use crate::data::initialize;
use crate::message::Message;
// use crate::page::PageType;

#[derive(Debug, Clone)]
pub struct DashPage {
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
            items: vec![],
            scroll: scrollable::State::new(),
            panes,
            panes_created: 2,
            focus: None,
            // night_mode: button::State::new(),
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Close(pane) => {
                if let Some((_, sibling)) = self.panes.close(&pane) {
                    self.focus = Some(sibling);
                }
            }
            _ => {}
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let DashPage { panes, .. } = self;

        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::new(&mut self.panes, |pane, content| {
            let is_focused = focus == Some(pane);

            let title_bar = pane_grid::TitleBar::new(format!("User Stats Pane {}", content.id))
                .padding(10)
                .style(style::TitleBar { is_focused });

            pane_grid::Content::new(content.view(pane, total_panes))
                .title_bar(title_bar)
                .style(style::Pane { is_focused })
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

        let mut user_info = Column::new()
            .spacing(5)
            .align_items(Align::Start)
            .max_width(150)
            .push(Text::new("Welcome to Fuzzr!!").size(16))
            .push(Text::new("TODO: Relevant user info here").size(14));

        if total_panes > 0 {
            user_info = user_info.push(button(
                close,
                "Close",
                Message::Close(pane),
                style::Button::Secondary,
            ));
        }

        let content = Scrollable::new(scroll)
            .width(Length::Fill)
            .spacing(10)
            .align_items(Align::Center)
            .push(user_info);

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
    use iced::{button, container, Background, Color, Vector};

    const BLACK: Color = Color::from_rgb(
        0xFF as f32 / 255.0,
        0xFF as f32 / 255.0,
        0xFF as f32 / 255.0,
    );

    const WHITE: Color = Color::from_rgb(
        0x00 as f32 / 255.0,
        0x00 as f32 / 255.0,
        0x00 as f32 / 255.0,
    );

    const ORANGE: Color = Color::from_rgb(
        0xFF as f32 / 255.0,
        0x45 as f32 / 255.0,
        0x00 as f32 / 255.0,
    );

    // pub enum Page {
    //     Background,
    // }

    pub struct TitleBar {
        pub is_focused: bool,
    }

    impl container::StyleSheet for TitleBar {
        fn style(&self) -> container::Style {
            let pane = Pane {
                is_focused: self.is_focused,
            }
            .style();

            container::Style {
                text_color: Some(BLACK),
                background: Some(pane.border_color.into()),
                ..Default::default()
            }
        }
    }

    pub struct Pane {
        pub is_focused: bool,
    }

    impl container::StyleSheet for Pane {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(BLACK)),
                border_width: 2.0,
                border_color: if self.is_focused {
                    Color::BLACK
                } else {
                    Color::from_rgb(0.7, 0.7, 0.7)
                },
                ..Default::default()
            }
        }
    }

    pub enum Button {
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            let (background, text_color) = match self {
                Button::Primary => (Some(BLACK), Color::WHITE),
                Button::Secondary => (Some(ORANGE), Color::WHITE),
            };

            button::Style {
                text_color,
                background: background.map(Background::Color),
                border_radius: 5.0,
                shadow_offset: Vector::new(0.0, 0.0),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            let background = match self {
                Button::Primary => Some(BLACK),
                Button::Secondary => Some(Color {
                    a: 0.2,
                    ..active.text_color
                }),
            };

            button::Style {
                background: background.map(Background::Color),
                ..active
            }
        }
    }
}
