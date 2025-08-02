use std::{collections::btree_map, io::Read};

use axum::{
    Error,
    extract::{Multipart, multipart::Field},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use http::StatusCode;
use uuid::Uuid;

use crate::service::image::{ImageBlank, ImageDetails, ImageService, ImageTypeConfig};

impl ImageService {
    pub async fn upload_image(
        multipart: Multipart,
        config: ImageTypeConfig,
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
