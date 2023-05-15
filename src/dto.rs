use serde::{Deserialize, Serialize};
use validator::{Validate};

#[derive(Serialize, Debug)]
pub struct ApiError {
    pub status_code: u16,
    pub errors: Vec<String>,
}

#[derive(Deserialize, Validate)]
pub struct AnimalRequestDto {
    #[validate(length(min = 3, message = "Species name must be at least 3 characters long"))]
    pub species: String,
    #[validate(length(min = 3, message = "Common name must be at least 3 characters long"))]
    pub common_name: String,
    #[validate(length(min = 3, message = "Habitat name must be at least 3 characters long"))]
    pub habitat: String,
    #[validate(range(min = 1, max = 100, message = "Lifespan must be between 1 and 100 years"))]
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
