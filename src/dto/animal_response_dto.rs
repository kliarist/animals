use serde::{Serialize};

#[derive(Serialize)]
pub struct AnimalResponseDto {
    pub id: i64,
    pub species: String,
    pub common_name: String,
    pub habitat: String,
    pub lifespan: i64,
    pub is_endangered: bool
}
