// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!A collection of known record types.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum KnownRecord {
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.actor.profile")]
    AppBskyActorProfile(Box<crate::app::bsky::actor::profile::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.feed.generator")]
    AppBskyFeedGenerator(Box<crate::app::bsky::feed::generator::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.feed.like")]
    AppBskyFeedLike(Box<crate::app::bsky::feed::like::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.feed.post")]
    AppBskyFeedPost(Box<crate::app::bsky::feed::post::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.feed.postgate")]
    AppBskyFeedPostgate(Box<crate::app::bsky::feed::postgate::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.feed.repost")]
    AppBskyFeedRepost(Box<crate::app::bsky::feed::repost::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.feed.threadgate")]
    AppBskyFeedThreadgate(Box<crate::app::bsky::feed::threadgate::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.graph.block")]
    AppBskyGraphBlock(Box<crate::app::bsky::graph::block::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.graph.follow")]
    AppBskyGraphFollow(Box<crate::app::bsky::graph::follow::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.graph.list")]
    AppBskyGraphList(Box<crate::app::bsky::graph::list::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.graph.listblock")]
    AppBskyGraphListblock(Box<crate::app::bsky::graph::listblock::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.graph.listitem")]
    AppBskyGraphListitem(Box<crate::app::bsky::graph::listitem::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.graph.starterpack")]
    AppBskyGraphStarterpack(Box<crate::app::bsky::graph::starterpack::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
    #[cfg(feature = "namespace-appbsky")]
    #[serde(rename = "app.bsky.labeler.service")]
    AppBskyLabelerService(Box<crate::app::bsky::labeler::service::Record>),
    #[cfg_attr(docsrs, doc(cfg(feature = "namespace-chatbsky")))]
    #[cfg(feature = "namespace-chatbsky")]
    #[serde(rename = "chat.bsky.actor.declaration")]
    ChatBskyActorDeclaration(Box<crate::chat::bsky::actor::declaration::Record>),
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::actor::profile::Record> for KnownRecord {
    fn from(record: crate::app::bsky::actor::profile::Record) -> Self {
        KnownRecord::AppBskyActorProfile(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::actor::profile::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::actor::profile::RecordData) -> Self {
        KnownRecord::AppBskyActorProfile(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::feed::generator::Record> for KnownRecord {
    fn from(record: crate::app::bsky::feed::generator::Record) -> Self {
        KnownRecord::AppBskyFeedGenerator(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::feed::generator::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::feed::generator::RecordData) -> Self {
        KnownRecord::AppBskyFeedGenerator(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::feed::like::Record> for KnownRecord {
    fn from(record: crate::app::bsky::feed::like::Record) -> Self {
        KnownRecord::AppBskyFeedLike(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::feed::like::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::feed::like::RecordData) -> Self {
        KnownRecord::AppBskyFeedLike(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::feed::post::Record> for KnownRecord {
    fn from(record: crate::app::bsky::feed::post::Record) -> Self {
        KnownRecord::AppBskyFeedPost(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::feed::post::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::feed::post::RecordData) -> Self {
        KnownRecord::AppBskyFeedPost(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::feed::postgate::Record> for KnownRecord {
    fn from(record: crate::app::bsky::feed::postgate::Record) -> Self {
        KnownRecord::AppBskyFeedPostgate(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::feed::postgate::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::feed::postgate::RecordData) -> Self {
        KnownRecord::AppBskyFeedPostgate(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::feed::repost::Record> for KnownRecord {
    fn from(record: crate::app::bsky::feed::repost::Record) -> Self {
        KnownRecord::AppBskyFeedRepost(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::feed::repost::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::feed::repost::RecordData) -> Self {
        KnownRecord::AppBskyFeedRepost(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::feed::threadgate::Record> for KnownRecord {
    fn from(record: crate::app::bsky::feed::threadgate::Record) -> Self {
        KnownRecord::AppBskyFeedThreadgate(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::feed::threadgate::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::feed::threadgate::RecordData) -> Self {
        KnownRecord::AppBskyFeedThreadgate(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::graph::block::Record> for KnownRecord {
    fn from(record: crate::app::bsky::graph::block::Record) -> Self {
        KnownRecord::AppBskyGraphBlock(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::graph::block::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::graph::block::RecordData) -> Self {
        KnownRecord::AppBskyGraphBlock(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::graph::follow::Record> for KnownRecord {
    fn from(record: crate::app::bsky::graph::follow::Record) -> Self {
        KnownRecord::AppBskyGraphFollow(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::graph::follow::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::graph::follow::RecordData) -> Self {
        KnownRecord::AppBskyGraphFollow(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::graph::list::Record> for KnownRecord {
    fn from(record: crate::app::bsky::graph::list::Record) -> Self {
        KnownRecord::AppBskyGraphList(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::graph::list::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::graph::list::RecordData) -> Self {
        KnownRecord::AppBskyGraphList(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::graph::listblock::Record> for KnownRecord {
    fn from(record: crate::app::bsky::graph::listblock::Record) -> Self {
        KnownRecord::AppBskyGraphListblock(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::graph::listblock::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::graph::listblock::RecordData) -> Self {
        KnownRecord::AppBskyGraphListblock(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::graph::listitem::Record> for KnownRecord {
    fn from(record: crate::app::bsky::graph::listitem::Record) -> Self {
        KnownRecord::AppBskyGraphListitem(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::graph::listitem::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::graph::listitem::RecordData) -> Self {
        KnownRecord::AppBskyGraphListitem(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::graph::starterpack::Record> for KnownRecord {
    fn from(record: crate::app::bsky::graph::starterpack::Record) -> Self {
        KnownRecord::AppBskyGraphStarterpack(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::graph::starterpack::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::graph::starterpack::RecordData) -> Self {
        KnownRecord::AppBskyGraphStarterpack(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::labeler::service::Record> for KnownRecord {
    fn from(record: crate::app::bsky::labeler::service::Record) -> Self {
        KnownRecord::AppBskyLabelerService(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-appbsky")))]
#[cfg(feature = "namespace-appbsky")]
impl From<crate::app::bsky::labeler::service::RecordData> for KnownRecord {
    fn from(record_data: crate::app::bsky::labeler::service::RecordData) -> Self {
        KnownRecord::AppBskyLabelerService(Box::new(record_data.into()))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-chatbsky")))]
#[cfg(feature = "namespace-chatbsky")]
impl From<crate::chat::bsky::actor::declaration::Record> for KnownRecord {
    fn from(record: crate::chat::bsky::actor::declaration::Record) -> Self {
        KnownRecord::ChatBskyActorDeclaration(Box::new(record))
    }
}
#[cfg_attr(docsrs, doc(cfg(feature = "namespace-chatbsky")))]
#[cfg(feature = "namespace-chatbsky")]
impl From<crate::chat::bsky::actor::declaration::RecordData> for KnownRecord {
    fn from(record_data: crate::chat::bsky::actor::declaration::RecordData) -> Self {
        KnownRecord::ChatBskyActorDeclaration(Box::new(record_data.into()))
    }
}