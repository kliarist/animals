use std::net::SocketAddr;
use std::sync::Arc;
use axum::{routing::{post, get, delete}, Router};
use tokio::sync::Mutex;
use crate::handler::animal_handler::{create, delete_by_id, find_all, find_by_id};

use crate::state::AppState;

mod state;
mod dto;
mod service;
mod mapper;
mod handler;
mod model;
mod repository;

#[tokio::main]
async fn main() {

    let app_state = Arc::new(Mutex::new(AppState::new()));

    let app = Router::new()
        .route("/animals", post(create))
        .route("/animals", get(find_all))
        .route("/animals/:animal_id", get(find_by_id))
        .route("/animals/:animal_id", delete(delete_by_id))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
