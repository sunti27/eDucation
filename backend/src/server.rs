use axum::Router;
use axum::routing::get;
use std::net::SocketAddr;
use sea_orm::DatabaseConnection;

use tower_http::trace::TraceLayer;

use crate::database;
use crate::api::teachers;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub async fn start(address: SocketAddr) {
    let state = AppState {
        conn: database::connect().await,
    };

    let app = Router::new()
        .route("/", get(sample_get))
        .nest("/teachers", teachers::get_router())
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