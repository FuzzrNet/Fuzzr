pub mod content;
pub mod feed;
pub mod publish;
pub mod testing;

#[derive(Debug, Clone)]
pub enum PageType {
  Feed,
  Publish,
  Content,
  Testing,
}
