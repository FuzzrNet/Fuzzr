use iced::{text_input, Element, Row, Text};

// use crate::data::content::ContentItem;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct ContentPage {
  pub input_state: text_input::State,
  pub input_value: String,
}

impl ContentPage {
  pub fn new() -> ContentPage {
    ContentPage {
      input_state: text_input::State::new(),
      input_value: String::new(),
    }
  }

  // fn update(&mut self, msg: Message, debug: &mut bool) {}

  pub fn view(&self) -> Element<Message> {
    Row::new()
      .push(Text::new("TODO: Content page").size(16))
      .into()
  }
}
