use iced::{
    alignment::Horizontal,
    pure::{column, container, row, text, Element},
    Alignment, Command, Length,
};
// use iced_native::ProgressBar;

// use crate::data::initialize;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct DashPage {
    // items: Vec<ContentItem>,
}

impl Default for DashPage {
    fn default() -> Self {
        Self::new()
    }
}

impl DashPage {
    pub fn new() -> DashPage {
        DashPage {
            // items: vec![],
        }
    }

    pub fn update(&mut self, _msg: Message) -> Command<Message> {
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        // let DashPage { .. } = self;

        let user_stats = column()
            .align_items(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(5)
            .padding(10)
            .push(text("User Stats").size(20))
            .push(text("Welcome to Fuzzr").size(16));

        // let value = 50.0;
        // let initialize = ProgressBar::new(0.0..=100.0, value);
        let spacer_row = row().height(Length::Fill);

        let fuzzr_stats = column()
            .align_items(Alignment::End)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(5)
            .padding(10)
            .push(text("Fuzzr Stats").size(20))
            .push(text("Fuzzr is still in pre-alpha").size(16))
            .push(spacer_row)
            .push(
                text("Initialization")
                    .size(12)
                    .horizontal_alignment(Horizontal::Center),
            )
            // .push(initialize)
            .padding(10);

        let dash_container = row().push(user_stats).push(fuzzr_stats);

        container(dash_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .into()
    }
}
