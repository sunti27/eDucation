use axum::Router;
use axum::routing::get;
use std::sync::Arc;
use std::net::SocketAddr;
use sea_orm::DatabaseConnection;

use tower_http::trace::TraceLayer;

use crate::database;

struct AppState {
    conn: DatabaseConnection,
}

pub async fn start(address: SocketAddr) {
    let state = Arc::new(AppState {
        conn: database::connect().await,
    });

    let app = Router::new()
        .route("/", get(sample_get))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    tracing::info!("Listening on http://{}", address);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Server failed");
}

async fn sample_get() -> &'static str {
    "Hello, World!"
}