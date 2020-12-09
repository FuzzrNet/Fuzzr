use iced::{
    Application, Color, Column, Command, Container, Element, Length, Settings, Subscription,
};
use iced_native::{window::Event::FileDropped, Event};

mod data;
mod message;
mod page;
mod ui;

use page::PageType;

use page::content::ContentPage;
use page::dashboard::DashPage;
use page::feed::FeedPage;
use page::publish::PublishPage;
use page::testing::TestingPage;

use message::Message;
use ui::page_selector::PageSelector;

pub fn main() -> iced::Result {
    pretty_env_logger::init();

    Fuzzr::run(Settings::default())
}

#[derive(Debug, Clone)]
struct Pages {
    dash: DashPage,
    feed: FeedPage,
    publish: PublishPage,
    content: ContentPage,
    testing: TestingPage,
}

#[derive(Debug, Clone)]
pub struct Fuzzr {
    pages: Pages, // All pages in the app
    current_page: PageType,
    page_buttons: PageSelector,
    background_color: Color,
}

impl Application for Fuzzr {
    type Executor = iced::executor::Default;
    type Message = message::Message;
    type Flags = ();

    fn new(_flags: ()) -> (Fuzzr, Command<Message>) {
        let pages = Pages {
            dash: DashPage::new(),
            feed: FeedPage::new(),
            publish: PublishPage::new(),
            content: ContentPage::new(),
            testing: TestingPage::new(),
        };

        (
            Fuzzr {
                pages,
                current_page: PageType::Dashboard,
                page_buttons: PageSelector::new(),
                background_color: Color::new(0.2, 0.2, 0.2, 1.0),
            },
            Command::none(),
        )
    }

    fn background_color(&self) -> Color {
        self.background_color
    }

    fn title(&self) -> String {
        "Fuzzr".to_string()
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        let mut update_page = |event: Message| match self.current_page {
            PageType::Dashboard => self.pages.dash.update(event),
            PageType::Feed => self.pages.feed.update(event),
            PageType::Publish => self.pages.publish.update(event),
            PageType::Content => self.pages.content.update(event),
            PageType::Testing => self.pages.testing.update(event),
        };

        match event {
            Message::PageChanged(page_type) => self.current_page = page_type,
            Message::FileDroppedOnWindow(_) => {
                update_page(event);
            }
            _ => {}
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events_with(|event, _status| match event {
            Event::Window(window_event) => match window_event {
                FileDropped(path) => Some(Message::FileDroppedOnWindow(path)),
                _ => None,
            },
            _ => None,
        })
    }

    fn view(&mut self) -> Element<Message> {
        let page: Element<_> = match self.current_page {
            PageType::Dashboard => self.pages.dash.view(),
            PageType::Feed => self.pages.feed.view(),
            PageType::Publish => self.pages.publish.view(),
            PageType::Content => self.pages.content.view(),
            PageType::Testing => self.pages.testing.view(),
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
