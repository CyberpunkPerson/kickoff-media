mod routes;
mod server;
mod operations;
mod di;
mod service;

#[tokio::main]
async fn main() {
    server::run().await;
}
