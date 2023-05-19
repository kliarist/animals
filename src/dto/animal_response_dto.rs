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

impl AnimalResponseDto {

    pub fn id(&self) -> i8 {
        self.id
    }

    pub fn species(&self) -> &String {
        &self.species
    }

    pub fn common_name(&self) -> &String {
        &self.common_name
    }

    pub fn habitat(&self) -> &String {
        &self.habitat
    }

    pub fn lifespan(&self) -> i8 {
        self.lifespan
    }

    pub fn is_endangered(&self) -> bool {
        self.is_endangered
    }
}