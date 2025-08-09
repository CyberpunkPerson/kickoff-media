use std::collections::HashMap;

use axum::{
    extract::Multipart,
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use http::StatusCode;
use uuid::Uuid;

use crate::service::{
    image::{AttachmentImageTypeConfig, AvatarImageTypeConfig, ImageBlank, ImageDetails, ImageType, ImageTypeConfig, ImageTypeConfigBundle},
    s3::client::S3Client,
};

pub struct ImageService {
    s3_client: S3Client,
    image_configs: HashMap<ImageType, ImageTypeConfigBundle>
}
impl ImageService {
    pub fn new(s3_client: S3Client) -> Self {
        
        let image_configs = HashMap::from([
            (ImageType::Avatar, ImageTypeConfigBundle::Avatar(AvatarImageTypeConfig::build())),
            (ImageType::Attachment, ImageTypeConfigBundle::Attachment(AttachmentImageTypeConfig::build()))
        ]);
        Self { s3_client, image_configs }
    }

    pub async fn upload_image(
        &self,
        multipart: Multipart,
        image_type: ImageType,
    ) -> Result<Vec<ImageDetails>, Response> {
        let image_blank = ImageBlank {
            id: Uuid::new_v4(),
            owner_id: Uuid::new_v4(),
            content: Self::extract_content(multipart).await?,
        };

        Ok(Vec::new())
    }

    async fn extract_content(mut multipart: Multipart) -> Result<Bytes, Response> {
        while let Some(field) = multipart.next_field().await.unwrap() {
            if let Some(name) = field.name() {
                if name == "file" {
                    return Ok(field.bytes().await.unwrap());
                }
            }
        }
        Err((StatusCode::BAD_REQUEST, "Wrong multipart format").into_response())
    }
}
