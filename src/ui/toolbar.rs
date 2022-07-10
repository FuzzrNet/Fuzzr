use iced::{
    pure::{button, column, container, row, text, Element},
    Color, Length,
};

use crate::message::Message;
use crate::page::PageType;
use crate::ui::style::{Theme, ThemeConfig};

#[derive(Debug, Clone)]
pub struct PageButton {
    label_text: String,
    page_type: PageType,
    is_disabled: bool,
}

#[derive(Debug, Clone)]
pub struct Toolbar {
    buttons: Vec<PageButton>,
    pub active_page: PageType,
}

fn inverse(theme: &Theme) -> Theme {
    match theme {
        Theme::Dark(ThemeConfig { selected, .. }) => Theme::Light(ThemeConfig {
            selected: selected.to_owned(),
            background: Color::BLACK,
            foreground: Color::WHITE,
        }),
        Theme::Light(ThemeConfig { selected, .. }) => Theme::Dark(ThemeConfig {
            selected: selected.to_owned(),
            background: Color::WHITE,
            foreground: Color::BLACK,
        }),
        Theme::Custom(ThemeConfig {
            selected,
            background,
            foreground,
        }) => Theme::Custom(ThemeConfig {
            selected: selected.to_owned(),
            background: foreground.to_owned(),
            foreground: background.to_owned(),
        }),
    }
}

fn selected(theme: &Theme, selected: bool) -> Theme {
    match theme {
        Theme::Dark(ThemeConfig {
            selected: _,
            background,
            foreground,
        }) => Theme::Dark(ThemeConfig {
            selected: selected.to_owned(),
            background: background.to_owned(),
            foreground: foreground.to_owned(),
        }),
        Theme::Light(ThemeConfig {
            selected: _,
            background,
            foreground,
        }) => Theme::Light(ThemeConfig {
            selected: selected.to_owned(),
            background: background.to_owned(),
            foreground: foreground.to_owned(),
        }),
        Theme::Custom(ThemeConfig {
            selected,
            background,
            foreground,
        }) => Theme::Custom(ThemeConfig {
            selected: selected.to_owned(),
            background: foreground.to_owned(),
            foreground: background.to_owned(),
        }),
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Toolbar {
    pub fn new() -> Toolbar {
        let buttons = vec![
            PageButton {
                label_text: "Dashboard".to_string(),
                page_type: PageType::Dashboard,
                is_disabled: true,
            },
            PageButton {
                label_text: "Feed".to_string(),
                page_type: PageType::Feed,
                is_disabled: true,
            },
            PageButton {
                label_text: "Publish".to_string(),
                page_type: PageType::Publish,
                is_disabled: false,
            },
            PageButton {
                label_text: "View".to_string(),
                page_type: PageType::View,
                is_disabled: false,
            },
            PageButton {
                label_text: "Site".to_string(),
                page_type: PageType::Site,
                is_disabled: true,
            },
            PageButton {
                label_text: "Settings".to_string(),
                page_type: PageType::Settings,
                is_disabled: false,
            },
        ];

        Toolbar {
            buttons,
            active_page: PageType::Publish,
        }
    }

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        let Toolbar {
            buttons,
            active_page,
        } = self;

        container(
            buttons
                .iter_mut()
                .fold(row(), |row, page_button| {
                    row.push(if page_button.is_disabled {
                        column().padding(2).push(
                            button(text(page_button.label_text.to_owned()).size(16)).style(*theme),
                        )
                    } else {
                        column().padding(2).push(
                            button(text(page_button.label_text.to_owned()).size(16))
                                .style(selected(theme, *active_page == page_button.page_type))
                                .on_press(Message::PageChanged(page_button.page_type.clone())),
                        )
                    })
                })
                .push(column().width(Length::Fill)) // spacer column
                .push(
                    button(text("Day/Night").size(16))
                        .style(*theme)
                        .on_press(Message::ThemeChanged(inverse(theme))),
                ),
        )
        .padding(10)
        .style(*theme)
        .into()
    }
}
