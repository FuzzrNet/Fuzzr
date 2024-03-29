use iced::{
    pure::{column, container, Application, Element},
    window, Alignment, Command, Length, Settings, Subscription, Theme,
};

use async_std::sync::{Arc, RwLock};
use iced_native::{
    window::Event::{FileDropped, Resized},
    Event,
};
use log::{error, info};
use std::path::PathBuf;

use fuzzr::{
    data::fs_ops::walk_dir,
    data::ipfs_client::{IpfsClient, IpfsClientRef},
    data::ipfs_ops::load_file,
    data::thumbnails,
    message::Message,
    page::dashboard::DashPage,
    page::feed::FeedPage,
    page::publish::PublishPage,
    page::settings::SettingsPage,
    page::site::SitePage,
    page::view::ViewPage,
    page::PageType,
    ui::toolbar::Toolbar,
};

async fn push_thumb_paths(
    paths: Vec<PathBuf>,
    publish_thumbs_paths: Arc<RwLock<Vec<PathBuf>>>,
) -> usize {
    let len = paths.len();
    publish_thumbs_paths.write().await.extend(paths);
    len
}

pub fn main() -> iced::Result {
    use std::env::{set_var, var};

    if var("RUST_LOG").is_err() {
        set_var("RUST_LOG", "fuzzr");
    }

    pretty_env_logger::init();

    Fuzzr::run(Settings {
        window: window::Settings {
            size: (800, 600),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
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
    toolbar: Toolbar,
    publish_thumbs_paths: Arc<RwLock<Vec<PathBuf>>>,
    theme: Theme,
}

impl Application for Fuzzr {
    type Executor = iced::executor::Default;
    type Message = fuzzr::message::Message;
    type Theme = Theme;
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
                toolbar: Toolbar::new(),
                ipfs_client: None,
                publish_thumbs_paths: Arc::new(RwLock::new(Vec::new())),
                theme: Theme::Dark,
            },
            Command::perform(IpfsClient::new(), Message::IpfsReady),
        )
    }

    // fn background_color(&self) -> Color {
    //     self.background_color
    // }

    fn title(&self) -> String {
        "Fuzzr".into()
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        Command::batch(vec![
            // Update all pages with all messages and batch any resulting commands.
            self.pages.dash.update(event.clone()),
            self.pages.feed.update(event.clone()),
            self.pages.publish.update(event.clone()),
            self.pages.view.update(event.clone()),
            self.pages.site.update(event.clone()),
            self.pages.settings.update(event.clone()),
            // Global message update handling
            match event {
                Message::PageChanged(page_type) => {
                    self.toolbar.active_page = page_type;
                    Command::none()
                }
                Message::IpfsReady(ipfs_client) => {
                    if let Ok(client) = ipfs_client {
                        self.ipfs_client = Some(Arc::new(RwLock::new(client)))
                    };
                    Command::none()
                }
                Message::FileDroppedOnWindow(path) => {
                    let paths = walk_dir(&path);
                    Command::perform(
                        push_thumb_paths(paths, Arc::clone(&self.publish_thumbs_paths)),
                        Message::ContentThumbProcessing,
                    )
                }
                // store_file(path, ipfs_client);
                // Command::perform(, Message::ContentDroppedOnWindow)
                // Command::perform(
                //     process_paths(Arc::clone(&self.thumbs)),
                //     Message::ContentThumbProcessed,
                // )
                // Command::none()
                Message::ContentAddedToIpfs(cid) => {
                    match cid {
                        Ok(maybe_cid) => {
                            match maybe_cid {
                                Some(cid) => {
                                    info!("Content successfully added to IPFS! Cid: {}", cid);
                                }
                                None => {
                                    error!("No CID was returned when attempting to store content in IPFS.");
                                }
                            }
                        }
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
                Message::ThemeChanged(theme) => {
                    self.theme = theme;
                    Command::none()
                }
                _ => Command::none(),
            },
        ])
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced_native::subscription::events_with(|event, _status| match event {
                Event::Window(window_event) => match window_event {
                    Resized { width, height } => Some(Message::WindowResized { width, height }),
                    FileDropped(path) => Some(Message::FileDroppedOnWindow(path)),
                    _ => None,
                },
                _ => None,
            }),
            thumbnails::process_paths(Arc::clone(&self.publish_thumbs_paths))
                .map(Message::PathThumbProgress),
        ])
    }

    fn view(&self) -> Element<Message> {
        let Fuzzr { pages, toolbar, .. } = self;

        let page: Element<_> = match toolbar.active_page {
            PageType::Dashboard => pages.dash.view(),
            PageType::Feed => pages.feed.view(),
            PageType::Publish => pages.publish.view(),
            PageType::View => pages.view.view(),
            PageType::Site => pages.site.view(),
            PageType::Settings => pages.settings.view(),
        };

        let content: Element<_> = column()
            .push(toolbar.view())
            .align_items(Alignment::Center)
            .push(page)
            .into();

        container(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme
    }
}
