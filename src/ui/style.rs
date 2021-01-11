use iced::{button, container, text_input};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light { selected: bool },
    Dark { selected: bool },
}

impl Theme {
    pub const ALL: [Theme; 2] = [
        Theme::Light { selected: false },
        Theme::Dark { selected: false },
    ];

    // let custom_theme = (foreground, background);
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Dark { selected: false }
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark { selected: _ } => dark::Container.into(),
            Theme::Light { selected: _ } => light::Container.into(),
        }
    }
}

impl From<Theme> for Box<dyn button::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark { selected } => Box::new(dark::Button { selected }),
            Theme::Light { selected } => Box::new(light::Button { selected }),
        }
    }
}

impl From<Theme> for Box<dyn text_input::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark { selected: _ } => dark::TextInput.into(),
            Theme::Light { selected: _ } => light::TextInput.into(),
        }
    }
}

mod light {
    use iced::{button, container, text_input, Background, Color};

    pub struct Button {
        pub selected: bool,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(if self.selected {
                    Color::BLACK
                } else {
                    Color::WHITE
                })),
                border_color: Color::BLACK,
                border_radius: 1.0,
                border_width: 1.0,
                text_color: if self.selected {
                    Color::WHITE
                } else {
                    Color::BLACK
                },
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(Color::BLACK)),
                border_color: Color::BLACK,
                border_radius: 1.0,
                border_width: 1.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }

        fn disabled(&self) -> button::Style {
            button::Style {
                text_color: Color::BLACK,
                ..button::Style::default()
            }
        }
    }

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Color {
                    a: 1.0,
                    ..Color::WHITE
                }
                .into(),
                text_color: Color::BLACK.into(),
                ..container::Style::default()
            }
        }
    }

    pub struct TextInput;

    impl text_input::StyleSheet for TextInput {
        fn active(&self) -> text_input::Style {
            text_input::Style {
                background: Color {
                    a: 1.0,
                    ..Color::WHITE
                }
                .into(),
                border_radius: 1.0,
                border_width: 1.0,
                border_color: Color::BLACK,
            }
        }

        fn focused(&self) -> text_input::Style {
            text_input::Style {
                border_width: 1.0,
                border_color: Color::BLACK,
                ..self.active()
            }
        }

        fn hovered(&self) -> text_input::Style {
            text_input::Style {
                border_width: 1.0,
                border_color: Color {
                    a: 1.0,
                    ..Color::BLACK
                },
                ..self.focused()
            }
        }

        fn placeholder_color(&self) -> Color {
            Color::BLACK
        }

        fn value_color(&self) -> Color {
            Color::BLACK
        }

        fn selection_color(&self) -> Color {
            Color::WHITE
        }
    }
}

mod dark {
    use iced::{button, container, text_input, Background, Color};

    pub struct Button {
        pub selected: bool,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(if self.selected {
                    Color::WHITE
                } else {
                    Color::BLACK
                })),
                border_color: Color::WHITE,
                border_radius: 1.0,
                border_width: 1.0,
                text_color: if self.selected {
                    Color::BLACK
                } else {
                    Color::WHITE
                },
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(Color::WHITE)),
                border_color: Color::WHITE,
                border_radius: 1.0,
                border_width: 1.0,
                text_color: Color::BLACK,
                ..button::Style::default()
            }
        }

        fn disabled(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Color {
                    a: 1.0,
                    ..Color::BLACK
                }
                .into(),
                text_color: Color::WHITE.into(),
                ..container::Style::default()
            }
        }
    }

    pub struct TextInput;

    impl text_input::StyleSheet for TextInput {
        fn active(&self) -> text_input::Style {
            text_input::Style {
                background: Color {
                    a: 1.0,
                    ..Color::BLACK
                }
                .into(),
                border_radius: 1.0,
                border_width: 1.0,
                border_color: Color::WHITE,
            }
        }

        fn focused(&self) -> text_input::Style {
            text_input::Style {
                border_width: 1.0,
                border_color: Color::WHITE,
                ..self.active()
            }
        }

        fn hovered(&self) -> text_input::Style {
            text_input::Style {
                border_width: 1.0,
                border_color: Color {
                    a: 1.0,
                    ..Color::WHITE
                },
                ..self.focused()
            }
        }

        fn placeholder_color(&self) -> Color {
            Color::WHITE
        }

        fn value_color(&self) -> Color {
            Color::WHITE
        }

        fn selection_color(&self) -> Color {
            Color::BLACK
        }
    }
}
