pub mod content;
pub mod feed;
pub mod publish;
pub mod testing;
pub mod dashboard;

#[derive(Debug, Clone)]
pub enum PageType {
  Dashboard,
  Feed,
  Publish,
  Content,
  Testing,
}
