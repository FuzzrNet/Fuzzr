use iced::{
    Align, Application, Color, Column, Command, Container, Element, Length, Settings, Subscription,
};
use iced_futures::futures;
use iced_futures::futures::channel::mpsc;
use iced_futures::futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use iced_native::{window::Event::FileDropped, Event};

use log::info;
use std::sync::{Arc, Mutex};

mod data;
mod message;
mod page;
mod ui;

use page::PageType;

use page::content::ContentPage;
use page::dashboard::DashPage;
use page::feed::FeedPage;
use page::publish::PublishPage;
use page::settings::SettingsPage;
use page::site::SitePage;
use page::testing::TestingPage;

use message::Message;
use ui::page_selector::PageSelector;

use data::content::{ContentItem, ContentItemBlock, PageContent};
use data::ipfs_client::IpfsClient;
use data::task_processor::Download;
use data::tasks::{IpfsAddFromFileTask, Task};

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
    site: SitePage,
    settings: SettingsPage,
    testing: TestingPage,
}

#[derive(Debug)]
pub struct Fuzzr {
    ipfs_client: Option<Arc<Mutex<IpfsClient>>>,
    task_sender: UnboundedSender<Task>,
    task_receiver: UnboundedReceiver<Task>,
    ipfs_ready: bool,
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
            site: SitePage::new(),
            settings: SettingsPage::new(),
            testing: TestingPage::new(),
        };

        let (task_sender, task_receiver) = mpsc::unbounded::<Task>();

        let app = Fuzzr {
            ipfs_client: None,
            task_sender,
            task_receiver,
            ipfs_ready: false,
            pages,
            current_page: PageType::Site,
            page_buttons: PageSelector::new(),
            background_color: Color::new(1.0, 1.0, 1.0, 1.0),
        };

        (app, Command::perform(IpfsClient::new(), Message::IpfsReady))
    }

    fn background_color(&self) -> Color {
        self.background_color
    }

    fn title(&self) -> String {
        "Fuzzr".into()
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        // Update all pages with all messages.
        self.pages.dash.update(event.clone());
        self.pages.feed.update(event.clone());
        self.pages.publish.update(event.clone());
        self.pages.content.update(event.clone());
        self.pages.site.update(event.clone());
        self.pages.settings.update(event.clone());
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
            Message::FileDroppedOnWindow(path) => {
                // Command::perform(ipfs.add_file_from_path(path), Message::ContentAddedToIpfs)
                self.task_sender
                    .unbounded_send(Task::IpfsAddFromFile(IpfsAddFromFileTask {
                        input: Some(path),
                        output: None,
                    }))
                    .unwrap();
                Command::none()
            }
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
            // Message::ContentPageLoadContent => {
            //     let cid_string = self.pages.content.input_value.clone();
            //     Command::perform(
            //         ipfs.get_bytes_from_cid_string(cid_string),
            //         Message::ContentPageImageLoaded,
            //     )
            //     Command::none()
            // }
            // Message::SitePagePublishButtonClicked => {
            //     info!(
            //         "Page publish button clicked with content: {}",
            //         self.pages.site.input_value
            //     );
            //     let content = self.pages.site.input_value.clone();
            //     let block = ContentItemBlock {
            //         content: ContentItem::Page(PageContent { content }),
            //     };
            //     // Command::perform(ipfs.add(&block), Message::ContentAddedToIpfs) // TODO
            //     Command::none()
            // }
            _ => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self {
            _ => iced_native::subscription::events_with(|event, _status| match event {
                Event::Window(window_event) => match window_event {
                    FileDropped(path) => Some(Message::FileDroppedOnWindow(path)),
                    _ => None,
                },
                _ => None,
            }),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let page: Element<_> = match self.current_page {
            PageType::Dashboard => self.pages.dash.view(),
            PageType::Feed => self.pages.feed.view(),
            PageType::Publish => self.pages.publish.view(),
            PageType::Content => self.pages.content.view(),
            PageType::Site => self.pages.site.view(),
            PageType::Settings => self.pages.settings.view(),
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
