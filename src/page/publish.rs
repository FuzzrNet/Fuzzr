use iced::{image, Column, Command, Container, Element, Image, Length, Text};

use log::{error, info};
use std::collections::btree_map::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::data::content::PathThumb;
use crate::data::thumbnails;
use crate::message::Message;

async fn lock_insert(
    publish_thumbs: Arc<Mutex<BTreeMap<PathBuf, PathThumb>>>,
    thumb: PathThumb,
    elapsed: Duration,
) {
    let mut publish_thumbs = publish_thumbs.lock().unwrap();

    info!(
        "thumbnailed {} items after {:.2?}",
        publish_thumbs.len(),
        elapsed
    );

    publish_thumbs.insert(thumb.path.clone(), thumb);
}

#[derive(Debug, Clone)]
pub struct PublishPage {
    // cid: Option<String>,
    publish_thumbs: Arc<Mutex<BTreeMap<PathBuf, PathThumb>>>,
}

impl PublishPage {
    pub fn new() -> PublishPage {
        PublishPage {
            publish_thumbs: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::ContentThumbProgress(progress) => match progress {
                thumbnails::Progress::Finished(thumb, elapsed) => Command::perform(
                    lock_insert(Arc::clone(&self.publish_thumbs), thumb, elapsed),
                    Message::ContentReadyToPublish,
                ),
                thumbnails::Progress::Error(error) => {
                    error!("{}", error);
                    Command::none()
                }
                thumbnails::Progress::Ready(unprocessed) => {
                    error!("Unprocessed {:?}", unprocessed);
                    Command::none()
                }
            },
            // Message::ContentAddedToIpfs(cid) => match cid {
            //     Ok(cid) => match cid {
            //         Some(cid) => self.cid = Some(cid.to_string()),
            //         None => {}
            //     },
            //     Err(_) => {}
            // },
            _ => Command::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let publish_thumbs = self.publish_thumbs.lock().unwrap();

        let drop_zone: Column<_> = if publish_thumbs.len() == 0 {
            publish_thumbs
                .iter()
                .fold(Column::new(), |col, (path, thumb)| {
                    col.push(Image::new(image::Handle::from_memory(thumb.image.clone())))
                })
        } else {
            Column::new().push(Text::new("Start adding content by dropping an image here"))
        };

        let publish_container = Column::new().push(drop_zone);

        Container::new(publish_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }
}
