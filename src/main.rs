use iced::{
    Align, Application, Color, Column, Command, Container, Element, Length, Settings, Subscription,
};
use iced_native::{window::Event::FileDropped, Event};

use async_std::sync::{Arc, Mutex};

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

use data::ipfs_client::IpfsClient;

pub fn main() -> iced::Result {
    pretty_env_logger::init();

    Fuzzr::run(Settings::default())
}

#[derive(Clone, Debug)]
struct Pages {
    dash: DashPage,
    feed: FeedPage,
    publish: PublishPage,
    content: ContentPage,
    testing: TestingPage,
}

#[derive(Clone, Debug)]
pub struct Fuzzr {
    ipfs_client: Option<Arc<Mutex<IpfsClient>>>,
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
                background_color: Color::new(1.0, 1.0, 1.0, 1.0),
                ipfs_client: None,
            },
            Command::perform(IpfsClient::new(), Message::IpfsReady),
        )
    }

    fn background_color(&self) -> Color {
        self.background_color
    }

    fn title(&self) -> String {
        "Fuzzr".to_string()
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        // Update all pages with all messages.
        self.pages.dash.update(event.clone());
        self.pages.feed.update(event.clone());
        self.pages.publish.update(event.clone());
        self.pages.content.update(event.clone());
        self.pages.testing.update(event.clone());

        match event {
            Message::PageChanged(page_type) => {
                self.current_page = page_type.to_owned();
                Command::none()
            }
            Message::IpfsReady(ipfs_client) => {
                match ipfs_client {
                    Ok(client) => self.ipfs_client = Some(Arc::new(Mutex::new(client))),
                    Err(_) => {}
                }
                Command::none()
            }
            Message::FileDroppedOnWindow(path) => match self.ipfs_client.clone() {
                Some(ipfs_client) => Command::perform(
                    IpfsClient::ipfs_add_file_from_path(ipfs_client, path),
                    Message::ContentAddedToIpfs,
                ),
                None => Command::none(),
            },
            Message::ContentAddedToIpfs(cid) => {
                match cid {
                    Ok(cid) => println!("Content successfully added to IPFS! Cid: {}", cid),
                    Err(err) => println!(
                        "Something went wrong when attempting to add content to IPFS. Error: {}",
                        err
                    ),
                }
                Command::none()
            }
            Message::ContentPageLoadContent => {
                let cid_string = self.pages.content.input_value.clone();
                match self.ipfs_client.clone() {
                    Some(ipfs_client) => Command::perform(
                        IpfsClient::ipfs_get(ipfs_client, cid_string),
                        Message::ContentPageImageLoaded,
                    ),
                    None => Command::none(),
                }
            }
            _ => Command::none(),
        }
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
            .spacing(20)
            .padding(20)
            .push(self.page_buttons.view())
            .align_items(Align::Center)
            .push(page)
            .into();

        Container::new(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_y()
            .into()
    }
}
