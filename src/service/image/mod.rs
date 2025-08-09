use std::collections::HashMap;

use crate::service::image::{attachment::AttachmentImageTypeConfig, avatar::AvatarImageTypeConfig};
use bytes::Bytes;
use config::Config;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

pub(crate) struct ImageDetails {
    file_name: String,
    image_size: ImageSizeType,
    content: Vec<u8>,
    width: u8,
    height: u8,
}
struct ImageBlank {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub content: Bytes,
}

 #[derive(PartialEq, Eq, Hash)]
pub(crate) enum ImageType {
    Attachment,
    Avatar,
}

enum ImageTypeConfigBundle{
    Attachment(AttachmentImageTypeConfig),
    Avatar(AvatarImageTypeConfig),
}

trait ImageTypeConfig: Sized + DeserializeOwned {
    const PATH: &'static str;

    fn original_size(&self) -> ImageSize;

    fn sizes(&self) -> HashMap<ImageSizeType, ImageSize>;

    fn build() -> Self {
        Config::builder()
            .add_source(config::File::with_name("config"))
            .build()
            .unwrap()
            .get(Self::PATH)
            .unwrap()
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash,
)]
enum ImageSizeType {
    #[serde(rename = "original")]
    Original,
    #[serde(rename = "big")]
    Big,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "tiny")]
    Tiny,
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct ImageSize {
    height: u8,
    width: u8,
}

mod attachment;
mod avatar;
pub mod service;
