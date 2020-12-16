// #[derive(Debug)]
// struct Initialize {
//     Idle { button: button::State },
//     Initializing { progress: f32 },
//     Finished { button: button::State },
//     Errored { button: button::State },
// }
// #[derive(Debug, Clone)]
// pub struct InitializeMessage {
//     Initialize,
//     InitializationProgressed(initialize::Progress),
// }

// impl Application for Initialize {
//     type Executor = executor::Default;
//     type Message = InitializeMessage;
//     type Flags = ();

//     fn new(_flags: ()) -> (Initialize, Command<InitializeMessage>) {
//         (
//             Initialize::Idle {
//                 button: button::State::new(),
//             },
//             Command::none(),
//         )
//     }

//     fn title(&self) -> String {
//         String::from("Fuzzr Initialization Progress")
//     }

//     fn update(&mut self, message: InitializeMessage) -> Command<InitializeMessage> {
//         match message {
//             InitializeMessage::Initialize => match self {
//                 Initialize::Idle { .. }
//                 | Initialize::Finished { .. }
//                 | Initialize::Errored { .. } => {
//                     *self = Initialize::Initializing { progress: 0.0 };
//                 }
//                 _ => {}
//             },
//             InitializeMessage::InitializationProgressed(message) => match self {
//                 Initialize::Initializing { progress } => match message {
//                     initialize::Progress::Started => {
//                         *progress = 0.0;
//                     }
//                     initialize::Progress::Advanced(percentage) => {
//                         *progress = percentage;
//                     }
//                     initialize::Progress::Finished => {
//                         *self = Initialize::Finished {
//                             button: button::State::new(),
//                         }
//                     }
//                     initialize::Progress::Errored => {
//                         *self = Initialize::Errored {
//                             button: button::State::new(),
//                         };
//                     }
//                 },
//                 _ => {}
//             },
//         };

//         Command::none()
//     }

//     fn subscription(&self) -> Subscription<InitializeMessage> {
//         match self {
//             Initialize::Initializing { .. } => {
//                 initialize::file("https://github.com/ipfs-embed/ipfs-embed.git")
//                     .map(InitializeMessage::InitializationProgressed)
//             }
//             _ => Subscription::none(),
//         }
//     }

//     fn view(&mut self) -> Element<InitializeMessage> {
//         let current_progress = match self {
//             Initialize::Idle { .. } => 0.0,
//             Initialize::Initializing { progress } => *progress,
//             Initialize::Finished { .. } => 100.0,
//             Initialize::Errored { .. } => 0.0,
//         };

//         let progress_bar = ProgressBar::new(0.0..=100.0, current_progress);

//         let control: Element<_> = match self {
//             Initialize::Idle { button } => Button::new(button, Text::new("Initialize Fuzzr"))
//                 .on_press(InitializeMessage::Initialize)
//                 .into(),
//             Initialize::Finished { .. } => Column::new()
//                 .spacing(10)
//                 .align_items(Align::Center)
//                 .push(Text::new("Initialization complete!"))
//                 .into(),
//             Initialize::Initializing { .. } => {
//                 Text::new(format!("Initializing... {:.2}%", current_progress)).into()
//             }
//             Initialize::Errored { button } => Column::new()
//                 .spacing(10)
//                 .align_items(Align::Center)
//                 .push(Text::new("Something went wrong :("))
//                 .push(
//                     Button::new(button, Text::new("Try again"))
//                         .on_press(InitializeMessage::Initialize),
//                 )
//                 .into(),
//         };

//         let content = Column::new()
//             .spacing(10)
//             .padding(10)
//             .align_items(Align::Center)
//             .push(progress_bar)
//             .push(control)
//             .push(Text::new(
//                 "This fetches and installs IPFS dependencies and only needs to be done once",
//             ));

//         Container::new(content)
//             .width(Length::Fill)
//             .height(Length::Fill)
//             .center_x()
//             .center_y()
//             .into()
//     }
// }
