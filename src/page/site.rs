use iced::{
    alignment::Horizontal,
    pure::{button, column, container, row, text, text_input, Element},
    Command, Length,
};

use crate::message::Message;
use crate::ui::style::Theme;

#[derive(Debug, Clone)]
pub struct SitePage {
    pub input_value: String,
}

impl Default for SitePage {
    fn default() -> Self {
        Self::new()
    }
}

impl SitePage {
    pub fn new() -> SitePage {
        SitePage {
            input_value: String::new(),
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::SitePageContentChanged(value) => {
                self.input_value = value;
                Command::none()
            }
            _ => Command::none(),
        }
    }

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        let page_header = column()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(5)
            .padding(10)
            .push(text("Modify Your Fuzzr Page").size(24));

        let fuzzr_page_content_input = text_input(
            "Edit Page Content",
            &self.input_value,
            Message::SitePageContentChanged,
        )
        .padding(15)
        .size(16)
        .on_submit(Message::SitePagePublishButtonClicked);

        let publish_page_button = button(text("Update and publish").size(16))
            .on_press(Message::SitePagePublishButtonClicked);

        let page_content = column()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(5)
            .padding(10)
            .push(
                text("Page Content")
                    .size(18)
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(fuzzr_page_content_input)
            .push(publish_page_button)
            .padding(10);

        let page_container = row().push(page_header).push(page_content);

        container(page_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .style(*theme)
            .into()
    }
}
