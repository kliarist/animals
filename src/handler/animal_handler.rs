use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use tokio::sync::Mutex;

use crate::dto::animal_request_dto::AnimalRequestDto;
use crate::dto::animal_response_dto::AnimalResponseDto;
use crate::config::app_state::AppState;

pub async fn find_all(State(app_state): State<Arc<Mutex<AppState>>>) -> (StatusCode, Json<Vec<AnimalResponseDto>>) {
    println!("find_all");
    return (StatusCode::OK, Json(app_state.lock().await.animal_service.find_all()))
}

pub async fn find_by_id(State(app_state): State<Arc<Mutex<AppState>>>, Path(id): Path<i32>) -> (StatusCode, Json<Value>) {
    println!("find_by_id");
    return app_state.lock().await.animal_service
        .find_by_id(id)
        .map_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"message":"animal not found"}))),
                     |animal| (StatusCode::OK, Json(json!(AnimalResponseDto::from(animal)))));
}

pub async fn create(app_state: State<Arc<Mutex<AppState>>>, Json(dto): Json<AnimalRequestDto>) -> (StatusCode, Json<i32>) {
    println!("create");
    return (StatusCode::CREATED, Json(app_state.lock().await.animal_service.save(dto)))
}

pub async fn delete_by_id(State(app_state): State<Arc<Mutex<AppState>>>, Path(id): Path<i32>) -> StatusCode {
    println!("delete_by_id");
    return app_state.lock().await.animal_service
        .delete_by_id(id)
        .map_or_else(|| StatusCode::NOT_FOUND, |_dto| {StatusCode::OK});
}
