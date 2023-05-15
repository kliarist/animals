mod dto;
mod model;
mod mapper;
mod handler;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use lazy_static::lazy_static;
use std::sync::Mutex;
use axum::routing::delete;

use crate::model::Animal;
use crate::handler::{create_animal, delete_animal, get_animal, get_animals};

//TODO remove when connected to DB
lazy_static! {
    static ref ANIMALS: Mutex<Vec<Animal>> = Mutex::new(vec![]);
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/animals", post(create_animal))
        .route("/animals", get(get_animals))
        .route("/animals/:animal_id", get(get_animal))
        .route("/animals/:animal_id", delete(delete_animal));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
