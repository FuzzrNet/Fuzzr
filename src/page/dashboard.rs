use iced::{Column, Element, Row, Text, scrollable};

use crate::data::content::ContentItem;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct DashPage {
  pub items: Vec<ContentItem>,
  pub scroll: scrollable::State,
}

impl DashPage {
  pub fn new() -> DashPage {
    DashPage {
      items: vec![],
      scroll: scrollable::State::new(),
    }
  }

  // fn update(&mut self, msg: Message) {
  //   match msg {
  //     Message::ContentPublished(content_item) => {
  //       let DashPage { items, .. } = self;
  //       items.push(content_item);
  //     }
  //     _ => {}
  //   }
  // }

  pub fn view(&self) -> Element<Message> {
    let dash_row = Column::new();

    let dash_row: Element<_> = dash_row
      .push(Text::new("Welcome to Fuzzr!!").size(18))
      .spacing(20)
      .padding(20)
      .push(Text::new("TODO: Relevant info for user here").size(14))
      .spacing(20)
      .padding(20)
      .into();

    dash_row
    }
}
