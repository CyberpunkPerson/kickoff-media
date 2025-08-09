use std::sync::Arc;

use crate::service::image::service::ImageService;
use crate::service::s3::{build_s3_client, client::S3Client};

#[derive(Clone)]
pub struct ApplicationState {
    pub image_service: Arc<ImageService>,
}

pub async fn build_state() -> ApplicationState {
    // s3
    let s3_client = S3Client::new(build_s3_client().await);
    let image_service = Arc::new(ImageService::new(s3_client));

    ApplicationState { image_service }
}
