use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Animal {
    id: i8,
    species: String,
    common_name: String,
    habitat: String,
    lifespan: i8,
    is_endangered: bool
}

impl Animal {
    pub fn new(id: i8, species: String, common_name: String, habitat: String, lifespan: i8, is_endangered: bool) -> Self {
        Self {
            id,
            species,
            common_name,
            habitat,
            lifespan,
            is_endangered
        }
    }

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