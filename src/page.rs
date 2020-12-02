pub mod content;
pub mod feed;
pub mod publish;

#[derive(Debug, Clone)]
pub enum PageType {
  Feed,
  Publish,
  Content,
}
