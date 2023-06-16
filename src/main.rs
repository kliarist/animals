use std::net::SocketAddr;
use std::sync::Arc;
use axum::{routing::{post, get, delete}, Router};
use dotenvy::dotenv;
use envconfig::Envconfig;
use tokio::sync::Mutex;
use crate::app_config::AppConfig;
use crate::handler::animal_handler::{create, delete_by_id, find_all, find_by_id};

use crate::app_state::AppState;

mod app_state;
mod dto;
mod service;
mod mapper;
mod handler;
mod model;
mod repository;
mod schemas;
mod db_util;
mod app_config;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = AppConfig::init_from_env().unwrap();
    let app_state = Arc::new(Mutex::new(AppState::new(&config)));

    let app = Router::new()
        .route("/animals", post(create))
        .route("/animals", get(find_all))
        .route("/animals/:animal_id", get(find_by_id))
        .route("/animals/:animal_id", delete(delete_by_id))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
