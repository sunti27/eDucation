use axum::Router;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::extract::{Json, State};
use crate::server::AppState;
use super::teacher_model as teacher;
use super::teacher_controller as controller;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(all_teachers).post(create_teacher))
}

async fn all_teachers(
    State(ref state): State<AppState>
) -> Result<(StatusCode, Json<Vec<teacher::Model>>), StatusCode> {
    let conn = &state.conn;

    let res = controller::all_teachers(&conn)
        .await;

    if res.is_ok() {
        Ok((StatusCode::OK, Json(res.unwrap())))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn create_teacher(
    State(ref state): State<AppState>,
    Json(teacher_body): Json<teacher::Model>
) -> StatusCode {
    let conn = &state.conn;

    let res = controller::create_teacher(&conn, teacher_body)
        .await;

    if res.is_ok() {
        StatusCode::CREATED
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
