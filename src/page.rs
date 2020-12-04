pub mod content;
pub mod dashboard;
pub mod feed;
pub mod publish;
pub mod testing;

#[derive(Debug, Clone)]
pub enum PageType {
    Dashboard,
    Feed,
    Publish,
    Content,
    Testing,
}
