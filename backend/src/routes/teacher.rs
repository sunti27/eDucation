use axum::Router;
use axum::http::StatusCode;
use axum::routing::post;
use axum::extract::{Json, State};
use crate::server::AppState;
use crate::entities::teacher;
use crate::controllers::teacher as teacher_controller;

pub fn get_router() -> Router {
    Router::new()
        .route("/", post(create_teacher))
}

async fn create_teacher(
    State(ref state): State<AppState>, 
    Json(teacher_body): Json<teacher::Model>
) -> StatusCode {
    let conn = &state.conn;

    teacher_controller::create_teacher(&conn, teacher_body)
        .await;

    StatusCode::CREATED
}
