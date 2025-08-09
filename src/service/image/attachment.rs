use std::collections::HashMap;

use serde::Deserialize;

use crate::service::image::ImageSizeType::Big;
use crate::service::image::ImageSizeType::Original;
use crate::service::image::ImageSizeType::Small;
use crate::service::image::ImageSizeType::Tiny;
use crate::service::image::{ImageSize, ImageSizeType, ImageTypeConfig};

#[derive(Debug, Deserialize)]
pub(super) struct AttachmentImageTypeConfig {
    original_size: ImageSize,
    big_size: ImageSize,
    small_size: ImageSize,
    tiny_size: ImageSize,
}

impl ImageTypeConfig for AttachmentImageTypeConfig {
    const PATH: &'static str = "attachment";

    fn sizes(&self) -> HashMap<ImageSizeType, ImageSize> {
        HashMap::from([
            (Original, self.original_size),
            (Big, self.big_size),
            (Small, self.small_size),
            (Tiny, self.tiny_size),
        ])
    }

    fn original_size(&self) -> ImageSize {
        self.original_size
    }
}
