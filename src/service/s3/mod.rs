use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client, config::Credentials};
use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct S3Config {
    region: Option<String>,
    host: String,
    access_key: String,
    secret_key: String,
}

impl S3Config {
    fn new() -> Self {
        Config::builder()
            .add_source(config::File::with_name("config"))
            .add_source(Environment::with_prefix("S3").separator("__"))
            .build()
            .unwrap()
            .get("s3")
            .unwrap()
    }
}

pub async fn build_s3_client() -> Client {
    let s3_config = S3Config::new();
    // let creds = Credentials::from_keys(&s3_config.access_key, &s3_config.secret_key, None);
    let creds = Credentials::new(
        &s3_config.access_key,
        &s3_config.secret_key,
        None,
        None,
        "no-matter",
    );
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(creds)
        .load()
        .await;

    let client_builder = aws_sdk_s3::config::Builder::from(&sdk_config);

    Client::from_conf(client_builder.build())
}

pub mod client;
