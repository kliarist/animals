use serde::{Serialize};

#[derive(Serialize)]
pub struct AnimalResponseDto {
    pub id: i32,
    pub species: String,
    pub common_name: String,
    pub habitat: String,
    pub lifespan: i32,
    pub is_endangered: bool
}
