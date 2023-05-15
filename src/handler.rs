use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use crate::ANIMALS;
use crate::dto::{AnimalRequestDto, AnimalResponseDto};
use crate::model::Animal;

pub async fn get_animals() -> (StatusCode, Json<Vec<AnimalResponseDto>>) {
    let animals = ANIMALS.lock().unwrap().iter()
        .map(|animal| AnimalResponseDto::from(animal))
        .collect();
    (StatusCode::OK, Json(animals))
}

pub async fn get_animal(Path(animal_id): Path<i8>) -> (StatusCode, Json<Value>) {
    let animals = ANIMALS.lock().unwrap();
    let animal_option = animals
        .iter()
        .find(|an| { an.id == animal_id });

    return match animal_option {
        Some(animal) => (StatusCode::OK, Json(json!(AnimalResponseDto::from(animal)))),
        None => (StatusCode::NOT_FOUND, Json(json!({"message":"animal not found"})))
    };
}

pub async fn create_animal(Json(dto): Json<AnimalRequestDto>) -> (StatusCode, Json<i8>) {
    let new_id: i8 = ANIMALS.lock().unwrap().len() as i8 + 1;
    let mut animal = Animal::from(&dto);
    animal.id = new_id;
    ANIMALS.lock().unwrap().push(animal);
    (StatusCode::CREATED, Json(new_id))
}

pub async fn delete_animal(Path(animal_id): Path<i8>) -> StatusCode  {
    let mut animals = ANIMALS.lock().unwrap();
    let index_option = animals
        .iter()
        .position(|an| { an.id == animal_id });

    return match index_option {
        Some(index) => {
            animals.remove(index);
            StatusCode::ACCEPTED
        },
        None => StatusCode::NOT_FOUND
    };
}
