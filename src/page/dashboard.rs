use iced::{
    Align, Column, Container, Element, HorizontalAlignment, Length, /* ProgressBar, */ Row, Text,
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
        // .push(initialize);

        Container::new(dash_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .into()
    }
}

//     fn view(&mut self) -> Element<Message> {
//         Column::new()
//             .padding(20)
//             .push(ProgressBar::new(0.0..=100.0, self.value))
//             // .push(
//             //     Slider::new(
//             //         &mut self.progress_bar_slider,
//             //         0.0..=100.0,
//             //         self.value,
//             //         Message::SliderChanged,
//             //     )
//             //     .step(0.01),
//             // )
//             .into()
//     }
// }

// mod style {

//     // There is none

//     use iced::{container, Color};

//     const BLACK: Color = Color::from_rgb(
//         0xFF as f32 / 255.0,
//         0xFF as f32 / 255.0,
//         0xFF as f32 / 255.0,
//     );

//     const WHITE: Color = Color::from_rgb(
//         0x00 as f32 / 255.0,
//         0x00 as f32 / 255.0,
//         0x00 as f32 / 255.0,
//     );

//     const ORANGE: Color = Color::from_rgb(
//         0xFF as f32 / 255.0,
//         0x45 as f32 / 255.0,
//         0x00 as f32 / 255.0,
//     );
// pub struct ContainerStyle;

// impl container::StyleSheet for ContainerStyle {
//     fn style(&self) -> Style {
//         let text_color = match self {
//             ContainerStyle::Text_Color => (Some(ORANGE))
//         }
//     }
// }}

// #[derive(Debug, Clone)]
// pub struct Contents {}

// impl Contents {
//     pub fn new(id: usize) -> Self {
//         Contents {}
//     }

//     fn view() -> Element<Message> {
//         // let Contents { .. } = self;
//         let button = |state, label, message, style| {
//             Button::new(
//                 state,
//                 Text::new(label)
//                     .width(Length::Fill)
//                     .horizontal_alignment(HorizontalAlignment::Center)
//                     .size(16),
//             )
//             .width(Length::Fill)
//             .padding(8)
//             .on_press(message)
//             .style(style)
//         };
//         let user_stats = Column::new()
//             .push(
//                 Row::new()
//                     .spacing(5)
//                     .padding(10)
//                     .push(Text::new("Welcome to Fuzzr!!").size(16)),
//             )
//             .push(
//                 Row::new()
//                     .spacing(5)
//                     .max_width(200)
//                     .padding(10)
//                     .push(Text::new("TODO: Relevant user info here").size(14)),
//             );
//     }
// }
