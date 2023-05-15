use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct Animal {
    pub id: i8,
    pub kind: String,
    pub age: i8,
    pub sound: String
}