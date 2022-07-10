use iced::{
    image::Handle,
    pure::{column, container, image, text, Element},
    Length,
};

use crate::data::content::ContentItem;
use crate::message::Message;

pub fn view(content: &ContentItem) -> Element<Message> {
    let content_container: Element<_> = match content {
        ContentItem::Image(image_content, _) => column()
            .push(image(Handle::from_memory(image_content.buffer.to_vec())))
            .into(),
        ContentItem::Text(text_content, _) => column().push(text(&text_content.string)).into(),
    };

    container(content_container)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .center_x()
        .into()
}
