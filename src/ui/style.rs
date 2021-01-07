use iced::container;

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
        Theme::Dark
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark => dark::Container.into(),
            Theme::Light => light::Container.into(),
        }
    }
}

mod light {
    use iced::{container, Color};

    const BACKGROUND: Color = Color::from_rgb(
        0xFF as f32 / 255.0,
        0xFF as f32 / 255.0,
        0xFF as f32 / 255.0,
    );

    // const ACCENT: Color = Color::from_rgb(
    //     0xDE as f32 / 255.0,
    //     0xFF as f32 / 255.0,
    //     0xE9 as f32 / 255.0,
    // );

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Color {
                    a: 1.0,
                    ..BACKGROUND
                }
                .into(),
                text_color: Color::BLACK.into(),
                ..container::Style::default()
            }
        }
    }
}

mod dark {
    use iced::{container, Color};

    const BACKGROUND: Color = Color::from_rgb(
        0x00 as f32 / 255.0,
        0x00 as f32 / 255.0,
        0x00 as f32 / 255.0,
    );
    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Color {
                    a: 1.0,
                    ..BACKGROUND
                }
                .into(),
                text_color: Color::WHITE.into(),
                ..container::Style::default()
            }
        }
    }
}
