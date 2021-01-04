use iced::{
    Align, Column, Container, Element, HorizontalAlignment, Length, /* ProgressBar, */ Row,
    Text,
};
// use iced_native::ProgressBar;

// use crate::data::initialize;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct DashPage {
    // items: Vec<ContentItem>,
// night_mode: button::State,
}

impl DashPage {
    pub fn new() -> DashPage {
        DashPage {
            // items: vec![],
            // night_mode: button::State::new(),
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            _ => {}
        }
    }

    pub fn view(&self) -> Element<Message> {
        // let DashPage { .. } = self;

        let user_stats = Column::new()
            .align_items(Align::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(5)
            .padding(10)
            .push(Text::new("User Stats").size(20))
            .push(Text::new("Welcome to Fuzzr").size(16));

        // let value = 50.0;
        // let initialize = ProgressBar::new(0.0..=100.0, value);
        let spacer_row = Row::new().height(Length::Fill);

        let fuzzr_stats = Column::new()
            .align_items(Align::End)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(5)
            .padding(10)
            .push(Text::new("Fuzzr Stats").size(20))
            .push(Text::new("Fuzzr is still in pre-alpha").size(16))
            .push(spacer_row)
            .push(
                Text::new("Initialization")
                    .size(12)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            // .push(initialize)
            .padding(10);

        let dash_container = Row::new().push(user_stats).push(fuzzr_stats);

        Container::new(dash_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .into()
    }
}
