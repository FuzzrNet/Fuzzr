use iced::{text_input, Element, Row, Text};

// use crate::data::content::ContentItem;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct PublishPage {
  pub input_state: text_input::State,
  pub input_value: String,
}

impl PublishPage {
  pub fn new() -> PublishPage {
    PublishPage {
      input_state: text_input::State::new(),
      input_value: String::new(),
    }
  }

  // fn update(&mut self, msg: Message, debug: &mut bool) {}

  pub fn view(&self) -> Element<Message> {
    Row::new()
      .push(Text::new("TODO: Publish page").size(16))
      .into()
  }
}
