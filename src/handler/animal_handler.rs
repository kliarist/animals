use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use log::info;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::app_state::AppState;
use crate::dto::animal_request_dto::AnimalRequestDto;
use crate::dto::animal_response_dto::AnimalResponseDto;

pub async fn find_all(
    State(app_state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Vec<AnimalResponseDto>>) {
    info!("Requesting find_all");
    (
        StatusCode::OK,
        Json(app_state.lock().await.animal_service.find_all()),
    )
}

pub async fn find_by_id(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<Value>) {
    info!("Requesting find_by_id with id: {}", id);
    app_state
        .lock()
        .await
        .animal_service
        .find_by_id(id)
        .map_or_else(
            || {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"message":"animal not found"})),
                )
            },
            |animal| (StatusCode::OK, Json(json!(animal))),
        )
}

pub async fn create(
    app_state: State<Arc<Mutex<AppState>>>,
    Json(dto): Json<AnimalRequestDto>,
) -> (StatusCode, Json<i32>) {
    info!("Requesting create with dto: {:?}", &dto);
    (
        StatusCode::CREATED,
        Json(app_state.lock().await.animal_service.save(dto)),
    )
}

pub async fn delete_by_id(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>,
) -> StatusCode {
    info!("Requesting delete_by_id with id: {}", id);
    app_state
        .lock()
        .await
        .animal_service
        .delete_by_id(id)
        .map_or_else(|| StatusCode::NOT_FOUND, |_dto| StatusCode::OK)
}
