use iced::{
    pure::{button, column, container, row, text, Element},
    Length, Theme,
};

use crate::message::Message;
use crate::page::PageType;

#[derive(Debug, Clone)]
pub struct PageButton {
    label_text: String,
    page_type: PageType,
    is_disabled: bool,
}

#[derive(Debug, Clone)]
pub struct Toolbar {
    buttons: Vec<PageButton>,
    theme: Theme,
    pub active_page: PageType,
}

fn invert(theme: &Theme, active: bool) -> Theme {
    if active {
        match theme {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    } else {
        *theme
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
            theme: Theme::Dark,
            active_page: PageType::Publish,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let Toolbar {
            buttons,
            theme,
            ..
            // active_page,
        } = self;

        container(
            buttons
                .iter()
                .fold(row(), |row, page_button| {
                    row.push(if page_button.is_disabled {
                        column()
                            .padding(2)
                            .push(button(text(page_button.label_text.to_owned()).size(16)))
                    } else {
                        column().padding(2).push(
                            button(text(page_button.label_text.to_owned()).size(16))
                                // .theme(invert(theme, *active_page == page_button.page_type))
                                .on_press(Message::PageChanged(page_button.page_type.clone())),
                        )
                    })
                })
                .push(column().width(Length::Fill)) // spacer column
                .push(
                    button(text("Day/Night").size(16))
                        .on_press(Message::ThemeChanged(invert(theme, true))),
                ),
        )
        .padding(10)
        .into()
    }
}
