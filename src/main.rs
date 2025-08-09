mod routes;
mod server;
mod operations;
mod state;
mod service;

#[tokio::main]
async fn main() {
    server::run().await;
}
