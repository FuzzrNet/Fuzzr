// IPFS application data formats

#[cfg(feature = "ipfs_rs")]
pub mod ipfs {
    use libipld::DagCbor;

    #[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
    pub struct ImageContent {
        pub buffer: Vec<u8>,
    }

    #[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
    pub enum ContentItem {
        Image(ImageContent),
    }
}

#[cfg(feature = "sled_db")]
pub mod ipfs {
    use libipld::DagCbor;

    #[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
    pub struct ImageContent {
        pub buffer: Vec<u8>,
    }

    #[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
    pub enum ContentItem {
        Image(ImageContent),
    }
}
