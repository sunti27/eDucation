mod server;
mod database;
mod api;

use std::net::SocketAddr;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    server::start(address).await;
}
