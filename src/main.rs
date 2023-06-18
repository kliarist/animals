use std::net::SocketAddr;
use std::sync::Arc;
use axum::{routing::{post, get, delete}, Router};
use dotenvy::dotenv;
use envconfig::Envconfig;
use tokio::sync::Mutex;
use crate::handler::animal_handler::{create, delete_by_id, find_all, find_by_id};
use crate::config::app_settings::AppSettings;
use crate::config::app_state::AppState;
use crate::db_util::{establish_connection, run_pending_migrations};

mod config;
mod db_util;
mod dto;
mod service;
mod mapper;
mod handler;
mod model;
mod repository;
mod schemas;

#[tokio::main]
async fn main() {
    // load settings
    dotenv().ok();
    let app_settings = AppSettings::init_from_env().unwrap();

    // establish db connection and run migrations
    let db_pool = establish_connection(app_settings.database_url.to_string());
    run_pending_migrations(db_pool.get().unwrap());

    //initialize app state
    let app_state = Arc::new(Mutex::new(AppState::new(db_pool)));

    println!("AppSettings: {:?}", app_settings);

    let app = Router::new()
        .route("/animals", post(create))
        .route("/animals", get(find_all))
        .route("/animals/:animal_id", get(find_by_id))
        .route("/animals/:animal_id", delete(delete_by_id))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], app_settings.server_port));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
