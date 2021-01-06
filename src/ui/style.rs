use iced::{button, container};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Light
    }
}

impl From<Theme> for Box<dyn button::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Button.into(),
        }
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Container.into(),
        }
    }
}

mod dark {
    use iced::{button, container, Color};

    const BACKGROUND: Color = Color::from_rgb(
        0x00 as f32 / 255.0,
        0x00 as f32 / 255.0,
        0x00 as f32 / 255.0,
    );

    const SURFACE: Color = Color::from_rgb(
        0xFF as f32 / 255.0,
        0xFF as f32 / 255.0,
        0xFF as f32 / 255.0,
    );

    // const ACCENT: Color = Color::from_rgb(
    //     0xDE as f32 / 255.0,
    //     0xFF as f32 / 255.0,
    //     0xE9 as f32 / 255.0,
    // );

    const ACTIVE: Color = Color::from_rgb(
        0xFF as f32 / 255.0,
        0xFF as f32 / 255.0,
        0xFF as f32 / 255.0,
    );

    // const SCROLLBAR: Color = Color::from_rgb(
    //     0x2E as f32 / 255.0,
    //     0x33 as f32 / 255.0,
    //     0x38 as f32 / 255.0,
    // );

    // const SCROLLER: Color = Color::from_rgb(
    //     0x20 as f32 / 255.0,
    //     0x22 as f32 / 255.0,
    //     0x25 as f32 / 255.0,
    // );
}

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color {
                a: 0.99,
                ..BACKGROUND
            }
            .into(),
            text_color: Color::WHITE.into(),
            ..container::Style::default()
        }
    }
}

pub struct Button;

impl button::StyleSheet for Button {
    fn style(&self) -> button::Style {
        button::Style {
            background: Color {
                a: 0.99,
                ..BACKGROUND
            }
            .into(),
            text_color: Color::WHITE.into(),
            ..button::Style::default()
        }
    }
}
