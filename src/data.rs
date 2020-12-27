pub mod initialize;

pub mod content;
pub mod ipfs_client;

// pub mod task_processor;
// mod tasks;

// use content::ContentItem;

// #[derive(Debug, Clone)]
// pub enum ContentMessage {
//   ItemPublished(ContentItem),
//   ItemViewed(ContentItem),
// }

#[cfg(feature = "ipfs-rs")]
pub mod ipfs {}

#[cfg(feature = "website")]
pub mod website {}
