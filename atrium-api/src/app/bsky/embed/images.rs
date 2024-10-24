// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.embed.images` namespace.
//!A set of images embedded in a Bluesky record (eg, a post).
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MainData {
    pub images: Vec<Image>,
}
pub type Main = crate::types::Object<MainData>;
///width:height represents an aspect ratio. It may be approximate, and may not correspond to absolute dimensions in any given unit.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AspectRatioData {
    pub height: core::num::NonZeroU64,
    pub width: core::num::NonZeroU64,
}
pub type AspectRatio = crate::types::Object<AspectRatioData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImageData {
    ///Alt text description of the image, for accessibility.
    pub alt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>,
    pub image: crate::types::BlobRef,
}
pub type Image = crate::types::Object<ImageData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewData {
    pub images: Vec<ViewImage>,
}
pub type View = crate::types::Object<ViewData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewImageData {
    ///Alt text description of the image, for accessibility.
    pub alt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>,
    ///Fully-qualified URL where a large version of the image can be fetched. May or may not be the exact original blob. For example, CDN location provided by the App View.
    pub fullsize: String,
    ///Fully-qualified URL where a thumbnail of the image can be fetched. For example, CDN location provided by the App View.
    pub thumb: String,
}
pub type ViewImage = crate::types::Object<ViewImageData>;
