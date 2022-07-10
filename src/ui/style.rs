use iced_native::{
    widget::{button, container, text_input},
    Background, Color,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeConfig {
    pub selected: bool,
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light(ThemeConfig),
    Dark(ThemeConfig),
    Custom(ThemeConfig),
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Dark(ThemeConfig {
            selected: false,
            background: Color::BLACK,
            foreground: Color::WHITE,
        })
    }
}

pub struct Button {
    pub selected: bool,
    pub background: Color,
    pub foreground: Color,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(if self.selected {
                self.foreground
            } else {
                self.background
            })),
            border_color: self.foreground,
            border_radius: 1.0,
            border_width: 1.0,
            text_color: if self.selected {
                self.background
            } else {
                self.foreground
            },
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.foreground)),
            border_color: self.foreground,
            border_radius: 1.0,
            border_width: 1.0,
            text_color: self.background,
            ..button::Style::default()
        }
    }

    fn disabled(&self) -> button::Style {
        button::Style {
            text_color: self.foreground,
            ..button::Style::default()
        }
    }
}

pub struct Container {
    background: Color,
    foreground: Color,
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color {
                a: 1.0,
                ..self.background
            }
            .into(),
            text_color: self.foreground.into(),
            ..container::Style::default()
        }
    }
}

pub struct ThemedTextInput {
    pub background: Color,
    pub foreground: Color,
}

impl text_input::StyleSheet for ThemedTextInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Color {
                a: 1.0,
                ..self.background
            }
            .into(),
            border_radius: 1.0,
            border_width: 1.0,
            border_color: self.foreground,
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            border_width: 1.0,
            border_color: self.foreground,
            ..self.active()
        }
    }

    fn hovered(&self) -> text_input::Style {
        text_input::Style {
            border_width: 1.0,
            border_color: Color {
                a: 1.0,
                ..self.foreground
            },
            ..self.focused()
        }
    }

    fn placeholder_color(&self) -> Color {
        self.foreground
    }

    fn value_color(&self) -> Color {
        self.foreground
    }

    fn selection_color(&self) -> Color {
        self.background
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark(ThemeConfig {
                selected: _,
                background,
                foreground,
            }) => Box::new(Container {
                background,
                foreground,
            }),
            Theme::Light(ThemeConfig {
                selected: _,
                background,
                foreground,
            }) => Box::new(Container {
                background,
                foreground,
            }),
            Theme::Custom(ThemeConfig {
                selected: _,
                background,
                foreground,
            }) => Box::new(Container {
                background,
                foreground,
            }),
        }
    }
}

impl From<Theme> for Box<dyn button::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark(ThemeConfig {
                selected,
                background,
                foreground,
            }) => Box::new(Button {
                selected,
                background,
                foreground,
            }),
            Theme::Light(ThemeConfig {
                selected,
                background,
                foreground,
            }) => Box::new(Button {
                selected,
                background,
                foreground,
            }),
            Theme::Custom(ThemeConfig {
                selected,
                background,
                foreground,
            }) => Box::new(Button {
                selected,
                background,
                foreground,
            }),
        }
    }
}

impl From<Theme> for Box<dyn text_input::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            // TODO: Do we need boxes?
            // TODO: Can we de-duplicate?
            Theme::Dark(ThemeConfig {
                selected: _,
                background,
                foreground,
            }) => Box::new(ThemedTextInput {
                background,
                foreground,
            }),
            Theme::Light(ThemeConfig {
                selected: _,
                background,
                foreground,
            }) => Box::new(ThemedTextInput {
                background,
                foreground,
            }),
            Theme::Custom(ThemeConfig {
                selected: _,
                background,
                foreground,
            }) => Box::new(ThemedTextInput {
                background,
                foreground,
            }),
        }
    }
}
