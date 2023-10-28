// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `app.bsky.feed.threadgate` namespace."]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<RecordAllowItem>>,
    pub created_at: String,
    pub post: String,
}
#[doc = "Allow replies from actors you follow."]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FollowingRule {}
#[doc = "Allow replies from actors on a list."]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListRule {
    pub list: String,
}
#[doc = "Allow replies from actors mentioned in your post."]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MentionRule {}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum RecordAllowItem {
    #[serde(rename = "app.bsky.feed.threadgate#mentionRule")]
    MentionRule(Box<MentionRule>),
    #[serde(rename = "app.bsky.feed.threadgate#followingRule")]
    FollowingRule(Box<FollowingRule>),
    #[serde(rename = "app.bsky.feed.threadgate#listRule")]
    ListRule(Box<ListRule>),
}