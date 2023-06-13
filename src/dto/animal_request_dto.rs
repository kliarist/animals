use serde::{Deserialize};
use validator::{Validate};

#[derive(Deserialize, Validate)]
pub struct AnimalRequestDto {
    #[validate(length(min = 3, message = "Species name must be at least 3 characters long"))]
    pub species: String,
    #[validate(length(min = 3, message = "Common name must be at least 3 characters long"))]
    pub common_name: String,
    #[validate(length(min = 3, message = "Habitat name must be at least 3 characters long"))]
    pub habitat: String,
    #[validate(range(min = 1, max = 100, message = "Lifespan must be between 1 and 100 years"))]
    pub lifespan: i64,
    pub is_endangered: bool
}

impl AnimalRequestDto {

    pub fn species(&self) -> &String {
        &self.species
    }

    pub fn common_name(&self) -> &String {
        &self.common_name
    }

    pub fn habitat(&self) -> &String {
        &self.habitat
    }

    pub fn lifespan(&self) -> i64 {
        self.lifespan
    }

    pub fn is_endangered(&self) -> bool {
        self.is_endangered
    }
}