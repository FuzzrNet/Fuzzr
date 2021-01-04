use iced::{
    button, text_input, Button, Column, Container, Element, HorizontalAlignment, Length, Row, Text,
    TextInput,
};

use crate::message::Message;

#[derive(Debug, Clone)]
pub struct SitePage {
    input_state: text_input::State,
    pub input_value: String,
    publish_page_button_state: button::State,
}

impl SitePage {
    pub fn new() -> SitePage {
        SitePage {
            input_state: text_input::State::new(),
            input_value: String::new(),
            publish_page_button_state: button::State::new(),
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SitePageContentChanged(value) => {
                self.input_value = value;
            }
            _ => {}
        };
    }

    pub fn view(&mut self) -> Element<Message> {
        let page_header = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(5)
            .padding(10)
            .push(Text::new("Modify Your Fuzzr Page").size(24));

        let fuzzr_page_content_input = TextInput::new(
            &mut self.input_state,
            "Edit Page Content",
            &self.input_value,
            Message::SitePageContentChanged,
        )
        .padding(15)
        .size(16)
        .on_submit(Message::SitePagePublishButtonClicked);

        let publish_page_button = Button::new(
            &mut self.publish_page_button_state,
            Text::new("Update and publish").size(16),
        )
        .on_press(Message::SitePagePublishButtonClicked);

        let page_content = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(5)
            .padding(10)
            .push(
                Text::new("Page Content")
                    .size(18)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .push(fuzzr_page_content_input)
            .push(publish_page_button)
            .padding(10);

        let page_container = Row::new().push(page_header).push(page_content);

        Container::new(page_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .into()
    }
}
