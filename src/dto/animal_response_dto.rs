use serde::{Serialize};

#[derive(Serialize)]
pub struct AnimalResponseDto {
    pub id: i8,
    pub species: String,
    pub common_name: String,
    pub habitat: String,
    pub lifespan: i8,
    pub is_endangered: bool
}
