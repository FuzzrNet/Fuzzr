pub mod dashboard;
pub mod feed;
pub mod publish;
pub mod settings;
pub mod site;
pub mod testing;
pub mod view;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageType {
    Dashboard,
    Feed,
    Publish,
    View,
    Site,
    Settings,
    Testing,
}
