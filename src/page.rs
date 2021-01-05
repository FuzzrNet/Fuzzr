pub mod dashboard;
pub mod feed;
pub mod publish;
pub mod settings;
pub mod site;
pub mod view;

#[derive(Debug, Clone, PartialEq)]
pub enum PageType {
    Dashboard,
    Feed,
    Publish,
    View,
    Site,
    Settings,
}
