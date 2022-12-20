use axum::Router;
use axum::routing::get;
use std::sync::Arc;
use std::net::SocketAddr;
use sea_orm::DatabaseConnection;

use tower_http::trace::TraceLayer;

use crate::database;
use crate::routes;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub async fn start(address: SocketAddr) {
    let state = AppState {
        conn: database::connect().await,
    };

    let app = Router::new()
        .nest("/teacher", routes::teacher::get_router())
        // .route("/", get(sample_get))
        
        
        .with_state(state)
        .layer(TraceLayer::new_for_http());
        

    tracing::info!("Listening on http://{}", address);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Server failed");
}

async fn sample_get() -> &'static str {
    "Hello, World!"
}