use iced::{button, Button, Element, Row, Text};

use crate::message::Message;
use crate::page::PageType;

#[derive(Debug, Clone)]
pub struct PageButton {
    label_text: String,
    button_state: button::State,
    page_type: PageType,
}

#[derive(Debug, Clone)]
pub struct PageSelector {
    buttons: Vec<PageButton>,
}

impl PageSelector {
    pub fn new() -> PageSelector {
        let mut buttons = vec![
            PageButton {
                label_text: "Dashboard".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Dashboard,
            },
            PageButton {
                label_text: "Feed".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Feed,
            },
            PageButton {
                label_text: "Publish".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Publish,
            },
            PageButton {
                label_text: "Content".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Content,
            },
        ];

        if std::env::var("RUST_LOG").unwrap_or_default() == "fuzzr" {
            let hidden_buttons = vec![
                PageButton {
                    label_text: "Site".to_string(),
                    button_state: button::State::new(),
                    page_type: PageType::Site,
                },
                PageButton {
                    label_text: "Settings".to_string(),
                    button_state: button::State::new(),
                    page_type: PageType::Settings,
                },
                PageButton {
                    label_text: "Testing".to_string(),
                    button_state: button::State::new(),
                    page_type: PageType::Testing,
                },
            ];

            buttons.extend(hidden_buttons);
        }

        PageSelector { buttons }
    }

    pub fn view(&mut self) -> Element<Message> {
        let PageSelector { buttons, .. } = self;

        buttons
            .into_iter()
            .fold(Row::new(), |row, page_button| {
                row.push(
                    Button::new(
                        &mut page_button.button_state,
                        Text::new(page_button.label_text.clone()).size(16),
                    )
                    .on_press(Message::PageChanged(page_button.page_type.clone())),
                )
            })
            .into()
    }
}
