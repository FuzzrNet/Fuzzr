use iced::{Application, Column, Command, Container, Element, Length, Settings};

mod data;
mod message;
mod page;
mod ui;

use page::PageType;

use page::content::ContentPage;
use page::feed::FeedPage;
use page::publish::PublishPage;

use message::Message;
use ui::page_selector::PageSelector;

pub fn main() -> iced::Result {
    env_logger::init();

    Fuzzr::run(Settings::default())
}

#[derive(Debug, Clone)]
struct Pages {
    feed: FeedPage,
    publish: PublishPage,
    content: ContentPage,
}

#[derive(Debug, Clone)]
pub struct Fuzzr {
    pages: Pages, // All pages in the app
    current_page: PageType,
    page_buttons: PageSelector,
}

impl Application for Fuzzr {
    type Executor = iced::executor::Default;
    type Message = message::Message;
    type Flags = ();

    fn new(_flags: ()) -> (Fuzzr, Command<Message>) {
        let pages = Pages {
            feed: FeedPage::new(),
            publish: PublishPage::new(),
            content: ContentPage::new(),
        };

        (
            Fuzzr {
                pages,
                current_page: PageType::Feed,
                page_buttons: PageSelector::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Fuzzr".to_string()
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        self.current_page = match event {
            Message::PageChanged(page_type) => page_type,
            _ => {
                // Page not found
                PageType::Feed
            }
        };

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let page: Element<_> = match self.current_page {
            PageType::Feed => self.pages.feed.view(),
            PageType::Publish => self.pages.publish.view(),
            PageType::Content => self.pages.content.view(),
        };

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(self.page_buttons.view())
            .push(page)
            .into();

        Container::new(content)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}
