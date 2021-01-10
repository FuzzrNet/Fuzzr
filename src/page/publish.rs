use iced::{Column, Container, Element, Length, Text};

use crate::message::Message;
use crate::ui::style::Theme;

#[derive(Debug, Clone)]
pub struct PublishPage {
    cid: Option<String>,
}

impl Default for PublishPage {
    fn default() -> Self {
        Self::new()
    }
}

impl PublishPage {
    pub fn new() -> PublishPage {
        PublishPage { cid: None }
    }

    pub fn update(&mut self, msg: Message) {
        if let Message::ContentAddedToIpfs(Ok(Some(cid))) = msg {
            self.cid = Some(cid.to_string())
        }
    }

    pub fn view(&self, theme: &Theme) -> Element<Message> {
        let drop_zone = match &self.cid {
            Some(cid) => Column::new().push(Text::new(format!(
                "{}\n\n(Clipboard copy not yet implemented, please check terminal)",
                cid
            ))), // Can't yet be copied to clipboard: https://github.com/hecrj/iced/issues/295
            // May want to put preview back, but we no longer have that data here
            None => Column::new().push(Text::new("Start adding content by dropping an image here")),
        };

        let publish_container = Column::new().push(drop_zone);

        Container::new(publish_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
            .style(*theme)
            .into()
    }
}
