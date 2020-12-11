use iced::{image, text_input, Column, Element, Image, Row, Text, TextInput};

// use crate::data::content::ContentItem;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct ContentPage {
    input_state: text_input::State,
    pub input_value: String,
    image: Option<image::Handle>,
}

impl ContentPage {
    pub fn new() -> ContentPage {
        ContentPage {
            input_state: text_input::State::new(),
            input_value: String::new(),
            image: None,
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::ContentPageInputChanged(value) => {
                self.input_value = value;
            }
            Message::ContentPageImageLoaded(buffer) => {
                self.image = match buffer {
                    Ok(image_data) => Some(image::Handle::from_memory(image_data)),
                    Err(_) => None,
                }
            }
            _ => {}
        };
    }

    pub fn view(&self) -> Element<Message> {

    pub fn view(&mut self) -> Element<Message> {
        let input = TextInput::new(
            &mut self.input_state,
            "Paste Content ID (CID) here",
            &self.input_value,
            Message::ContentPageInputChanged,
        )
        .padding(15)
        .size(16)
        .on_submit(Message::ContentPageLoadContent);

        let content_image = match &self.image {
            Some(image) => Column::new().push(Image::new(image.clone())),
            None => Column::new().push(Text::new(
                "Start adding content by dropping the file or folder here",
            )),
        };

        let content_container = Row::new().push(input).push(content_image).into()

        Container::new(content_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .into()
    }
}
