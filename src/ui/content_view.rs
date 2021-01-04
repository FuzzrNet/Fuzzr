use iced::{image, Column, Container, Element, Image, Length, Text};

use crate::data::content::ContentItem;
use crate::message::Message;

pub fn view(content: &ContentItem) -> Element<Message> {
    let content_container: Element<_> = match content {
        ContentItem::Image(image_content, _) => Column::new()
            .push(Image::new(image::Handle::from_memory(
                image_content.buffer.clone(),
            )))
            .into(),
        ContentItem::Text(text_content, _) => {
            Column::new().push(Text::new(&text_content.string)).into()
        }
    };

    Container::new(content_container)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .center_x()
        .into()
}
