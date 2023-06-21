use crate::config::app_settings::AppSettings;
use crate::config::app_state::AppState;
use crate::db_util::{establish_connection, run_pending_migrations};
use crate::handler::animal_handler::{create, delete_by_id, find_all, find_by_id};
use axum::{
    routing::{delete, get, post},
    Router,
};
use dotenvy::dotenv;
use env_logger::Env;
use envconfig::Envconfig;
use log::info;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

mod config;
mod db_util;
mod dto;
mod handler;
mod mapper;
mod model;
mod repository;
mod schemas;
mod service;

#[tokio::main]
async fn main() {
    // load settings
    dotenv().ok();
    let app_settings = AppSettings::init_from_env().unwrap();

    // initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!(
        "Logger initialized with level \"{}\"",
        app_settings.log_level()
    );

    // establish db connection and run migrations
    let db_pool = establish_connection(app_settings.database_url().to_string());
    info!(
        "Database connection established at \"{}\"",
        app_settings.database_url()
    );
    run_pending_migrations(db_pool.get().unwrap());
    info!("Database migrations run");

    //initialize app state
    let app_state = Arc::new(Mutex::new(AppState::new(db_pool)));

    let app = Router::new()
        .route("/animals", post(create))
        .route("/animals", get(find_all))
        .route("/animals/:animal_id", get(find_by_id))
        .route("/animals/:animal_id", delete(delete_by_id))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], app_settings.server_port()));

    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
