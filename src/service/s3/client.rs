use aws_sdk_s3::Client;
use bytes::Bytes;

pub struct S3Client {
    s3_client: Client
}

impl S3Client {
    pub fn new(s3_client: Client) -> Self{
        Self{s3_client: s3_client}
    }

    pub async fn upload(bucket_name: String, image: Bytes) {
        
    }
}
