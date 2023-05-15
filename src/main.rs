mod dto;
mod model;

use axum::{routing::{get, post}, Router, Json};
use std::net::SocketAddr;
use axum::http::StatusCode;
use lazy_static::lazy_static;
use std::sync::Mutex;


use crate::dto::{AnimalRequestDto, AnimalResponseDto};
use crate::model::Animal;

lazy_static! {
    static ref ANIMALS: Mutex<Vec<Animal>> = Mutex::new(vec![]);
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/animals", get(get_animals))
        .route("/animals", post(create_animal));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_animals() -> (StatusCode, Json<Vec<AnimalResponseDto>>) {
    let animals = ANIMALS.lock().unwrap().iter().map(|x| {
        return AnimalResponseDto {
            id: x.id,
            age: x.age,
            kind: x.kind.to_string(),
            sound: x.sound.to_string()
        };
    }).collect();
    (StatusCode::OK, Json(animals))
}

async fn create_animal(Json(body): Json<AnimalRequestDto>) -> (StatusCode, Json<i8>) {
    let new_id: i8 = 1;
    //mapper
    let animal = Animal {
        id: new_id,
        age: body.age,
        kind: body.kind,
        sound: body.sound
    };

    ANIMALS.lock().unwrap().push(animal);

    (StatusCode::CREATED, Json(new_id))
}