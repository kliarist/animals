use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct ApiError {
    pub status_code: u16,
    pub errors: Vec<String>,
}

#[derive(Deserialize)]
pub struct AnimalRequestDto {
    pub species: String,
    pub common_name: String,
    pub habitat: String,
    pub lifespan: i8,
    pub is_endangered: bool
}

#[derive(Serialize)]
pub struct AnimalResponseDto {
    pub id: i8,
    pub species: String,
    pub common_name: String,
    pub habitat: String,
    pub lifespan: i8,
    pub is_endangered: bool
}