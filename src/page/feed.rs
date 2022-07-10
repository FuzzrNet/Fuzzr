use iced::{Column, Command, Element, Length, Text};
use iced_native::widget::container::Container;

// use crate::data::content::ContentItem;
use crate::message::Message;
use crate::ui::style::Theme;

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

    pub fn view(&self, theme: &Theme) -> Element<Message> {
        let feed_container = Column::new().push(Text::new("TODO: Feed page").size(18));

        Container::new(feed_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .style(*theme)
            .into()
    }
}
