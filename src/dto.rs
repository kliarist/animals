use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AnimalRequestDto {
    pub kind: String,
    pub age: i8,
    pub sound: String
}

#[derive(Serialize)]
pub struct AnimalResponseDto {
    pub id: i8,
    pub kind: String,
    pub age: i8,
    pub sound: String
}