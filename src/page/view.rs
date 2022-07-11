use iced::{
    pure::{column, container, text_input, Element},
    Command, Length,
};
use log::error;

use crate::data::content::ContentItem;
use crate::message::Message;
use crate::ui::content_view;

#[derive(Debug, Clone)]
pub struct ViewPage {
    pub input_value: String,
    content: Option<ContentItem>,
}

impl Default for ViewPage {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewPage {
    pub fn new() -> ViewPage {
        ViewPage {
            input_value: String::new(),
            content: None,
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::ViewPageInputChanged(value) => {
                self.input_value = value;
                Command::none()
            }
            Message::ViewPageContentLoaded(content_item) => match content_item {
                Ok(content_item) => {
                    self.content = Some(content_item);
                    Command::none()
                }
                Err(err) => {
                    error!("Error loading content item from IPFS: {}", err);
                    Command::none()
                }
            },
            _ => Command::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let input = text_input(
            "Paste Content ID (CID) here",
            &self.input_value,
            Message::ViewPageInputChanged,
        )
        .padding(15)
        .size(16)
        .on_submit(Message::ViewPageLoadContent);

        let content_image = match &self.content {
            Some(content) => column().push(content_view::view(content)),
            None => column(),
        };

        let content_container = column().push(input).push(content_image);

        container(content_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .into()
    }
}
