use iced::{
    Align, Application, Color, Column, Command, Container, Element, Length, Settings, Subscription,
};
use iced_native::{window::Event::FileDropped, Event};

use async_std::sync::{Arc, Mutex};
use log::{error, info};
use std::collections::btree_map::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc as SyncArc, Mutex as SyncMutex};

mod data;
mod message;
mod page;
mod ui;

use page::PageType;

use page::dashboard::DashPage;
use page::feed::FeedPage;
use page::publish::PublishPage;
use page::settings::SettingsPage;
use page::site::SitePage;
use page::view::ViewPage;

use message::Message;
use ui::page_selector::PageSelector;

use data::content::PathThumb;
use data::fs_ops::walk_dir;
use data::ipfs_client::{IpfsClient, IpfsClientRef};
use data::ipfs_ops::{load_file, store_file};
use data::thumbnails;

pub fn main() -> iced::Result {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "fuzzr");
    }

    pretty_env_logger::init();

    Fuzzr::run(Settings::default())
}

#[derive(Clone, Debug)]
struct Pages {
    dash: DashPage,
    feed: FeedPage,
    publish: PublishPage,
    view: ViewPage,
    site: SitePage,
    settings: SettingsPage,
}

#[derive(Clone, Debug)]
pub struct Fuzzr {
    ipfs_client: Option<IpfsClientRef>,
    pages: Pages, // All pages in the app
    current_page: PageType,
    page_buttons: PageSelector,
    background_color: Color,
    publish_thumbs_paths: SyncArc<SyncMutex<Vec<PathBuf>>>,
    publish_thumbs: Arc<Mutex<BTreeMap<PathBuf, PathThumb>>>,
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
            view: ViewPage::new(),
            site: SitePage::new(),
            settings: SettingsPage::new(),
        };

        (
            Fuzzr {
                pages,
                current_page: PageType::Publish, // Default page
                page_buttons: PageSelector::new(),
                background_color: Color::new(1.0, 1.0, 1.0, 1.0),
                ipfs_client: None,
                publish_thumbs_paths: SyncArc::new(SyncMutex::new(Vec::new())),
                publish_thumbs: Arc::new(Mutex::new(BTreeMap::new())),
            },
            Command::perform(IpfsClient::new(), Message::IpfsReady),
        )
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
        self.pages.view.update(event.clone());
        self.pages.site.update(event.clone());
        self.pages.settings.update(event.clone());

        match event {
            Message::PageChanged(page_type) => {
                self.current_page = page_type.clone();
                self.page_buttons.active_page = page_type.clone();
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
                let paths = walk_dir(&path);
                let mut publish_thumbs_paths = self.publish_thumbs_paths.lock().unwrap();
                publish_thumbs_paths.extend(paths);
                Command::none()
            }
            // store_file(path, ipfs_client);
            // Command::perform(, Message::ContentDroppedOnWindow)
            // Command::perform(
            //     process_paths(Arc::clone(&self.thumbs)),
            //     Message::ContentThumbProcessed,
            // )
            // Command::none()
            Message::ContentThumbProgress(progress) => {
                match progress {
                    thumbnails::Progress::Finished(thumb) => {
                        // info!("processed results with length: {}", result.len());
                        info!("processed results with length: {}", thumb.image.len());
                    }
                    thumbnails::Progress::Error(error) => {
                        error!("{}", error);
                    }
                    thumbnails::Progress::Ready(unprocessed) => {
                        error!("Unprocessed {:?}", unprocessed);
                    }
                }
                Command::none()
            }
            Message::ContentAddedToIpfs(cid) => {
                match cid {
                    Ok(maybe_cid) => match maybe_cid {
                        Some(cid) => {
                            info!("Content successfully added to IPFS! Cid: {}", cid);
                        }
                        None => {
                            error!("No CID was returned when attempting to store content in IPFS.");
                        }
                    },
                    Err(err) => {
                        error!("Something went wrong when attempting to add content to IPFS. Error: {}", err);
                    }
                }
                Command::none()
            }
            Message::ViewPageLoadContent => {
                let cid_string = self.pages.view.input_value.clone();
                match self.ipfs_client.clone() {
                    Some(ipfs_client) => Command::perform(
                        load_file(cid_string, ipfs_client),
                        Message::ViewPageContentLoaded,
                    ),
                    None => Command::none(),
                }
            }
            _ => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut subscriptions =
            vec![iced_native::subscription::events_with(
                |event, _status| match event {
                    Event::Window(window_event) => match window_event {
                        FileDropped(path) => Some(Message::FileDroppedOnWindow(path)),
                        _ => None,
                    },
                    _ => None,
                },
            )];

        let publish_thumbs_paths = &mut self.publish_thumbs_paths.lock().unwrap();
        if !publish_thumbs_paths.is_empty() {
            println!("HERE: {:?}", &publish_thumbs_paths);
            subscriptions.push(
                thumbnails::process_paths(publish_thumbs_paths.to_vec())
                    .map(Message::ContentThumbProgress),
            );
            publish_thumbs_paths.clear();
        }

        Subscription::batch(subscriptions)
    }

    fn view(&mut self) -> Element<Message> {
        let page: Element<_> = match self.current_page {
            PageType::Dashboard => self.pages.dash.view(),
            PageType::Feed => self.pages.feed.view(),
            PageType::Publish => self.pages.publish.view(),
            PageType::View => self.pages.view.view(),
            PageType::Site => self.pages.site.view(),
            PageType::Settings => self.pages.settings.view(),
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
