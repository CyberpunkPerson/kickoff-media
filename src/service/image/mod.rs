use bytes::Bytes;
use uuid::Uuid;

pub struct ImageService {
    image_type_config: ImageTypeConfig
    
}
pub struct ImageDetails {
    file_name: String,
    image_size: ImageSize,
    content: Vec<u8>,
    width: u32,
    height: u32,
}

pub struct ImageBlank {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub content: Bytes,
}

pub struct ImageTypeConfig {}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum ImageSize {
    #[serde(rename = "original")]
    Original,
    #[serde(rename = "big")]
    Big,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "tiny")]
    Tiny,
}

mod service;
