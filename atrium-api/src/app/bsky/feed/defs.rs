// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `app.bsky.feed.defs` namespace.

// app.bsky.feed.defs#feedViewPost
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeedViewPost {
    pub post: crate::app::bsky::feed::defs::PostView,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Box<FeedViewPostReasonEnum>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<ReplyRef>,
}

// app.bsky.feed.defs#notFoundPost
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NotFoundPost {
    pub not_found: bool,
    pub uri: String,
}

// app.bsky.feed.defs#postView
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostView {
    pub author: crate::app::bsky::actor::defs::ProfileViewBasic,
    pub cid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Box<PostViewEmbedEnum>>,
    pub indexed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i32>,
    pub record: crate::records::Record,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repost_count: Option<i32>,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,
}

// app.bsky.feed.defs#reasonRepost
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReasonRepost {
    pub by: crate::app::bsky::actor::defs::ProfileViewBasic,
    pub indexed_at: String,
}

// app.bsky.feed.defs#replyRef
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReplyRef {
    pub parent: crate::app::bsky::feed::defs::PostView,
    pub root: crate::app::bsky::feed::defs::PostView,
}

// app.bsky.feed.defs#threadViewPost
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ThreadViewPost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<ThreadViewPostParentEnum>>,
    pub post: PostView,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<ThreadViewPostRepliesItem>>,
}

// app.bsky.feed.defs#viewerState
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViewerState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repost: Option<String>,
}

#[allow(clippy::large_enum_variant)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "$type")]
pub enum FeedViewPostReasonEnum {
    #[serde(rename = "app.bsky.feed.defs#reasonRepost")]
    ReasonRepost(ReasonRepost),
}

#[allow(clippy::large_enum_variant)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "$type")]
pub enum PostViewEmbedEnum {
    #[serde(rename = "app.bsky.embed.images#view")]
    AppBskyEmbedImagesView(crate::app::bsky::embed::images::View),
    #[serde(rename = "app.bsky.embed.external#view")]
    AppBskyEmbedExternalView(crate::app::bsky::embed::external::View),
    #[serde(rename = "app.bsky.embed.record#view")]
    AppBskyEmbedRecordView(crate::app::bsky::embed::record::View),
    #[serde(rename = "app.bsky.embed.recordWithMedia#view")]
    AppBskyEmbedRecordWithMediaView(crate::app::bsky::embed::record_with_media::View),
}

#[allow(clippy::large_enum_variant)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "$type")]
pub enum ThreadViewPostParentEnum {
    #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
    ThreadViewPost(ThreadViewPost),
    #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
    NotFoundPost(NotFoundPost),
}

#[allow(clippy::large_enum_variant)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "$type")]
pub enum ThreadViewPostRepliesItem {
    #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
    ThreadViewPost(ThreadViewPost),
    #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
    NotFoundPost(NotFoundPost),
}