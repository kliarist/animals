use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Animal {
    pub id: i8,
    pub species: String,
    pub common_name: String,
    pub habitat: String,
    pub lifespan: i8,
    pub is_endangered: bool
}