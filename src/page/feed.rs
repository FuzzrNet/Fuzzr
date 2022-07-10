use iced::{
    pure::{column, container, text, Element},
    Command, Length,
};

// use crate::data::content::ContentItem;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct FeedPage {
    // items: Vec<ContentItem>,
    // scroll: scrollable::State,
}

impl Default for FeedPage {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedPage {
    pub fn new() -> FeedPage {
        FeedPage {
            // items: vec![],
            // scroll: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, _msg: Message) -> Command<Message> {
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let feed_container = column().push(text("TODO: Feed page").size(18));

        container(feed_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .into()
    }
}
