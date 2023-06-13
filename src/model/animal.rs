use diesel::{Queryable, Selectable, Insertable};
use serde::{Serialize, Deserialize};
use crate::schemas::schema::animals;


#[derive(Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = animals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Animal {
    id: i32,
    species: String,
    common_name: String,
    habitat: String,
    lifespan: i32,
    is_endangered: bool
}

impl Animal {
    pub fn new(id: i32, species: String, common_name: String, habitat: String, lifespan: i32, is_endangered: bool) -> Self {
        Self {
            id,
            species,
            common_name,
            habitat,
            lifespan,
            is_endangered
        }
    }

    pub fn id(&self) -> i32 {
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

    pub fn lifespan(&self) -> i32 {
        self.lifespan
    }

    pub fn is_endangered(&self) -> bool {
        self.is_endangered
    }
}