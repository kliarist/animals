use std::collections::HashMap;

use crate::model::Animal;

#[derive(Clone)]
pub struct AnimalRepository {
    animals: HashMap<i8, Animal>
}

impl AnimalRepository {

    pub fn new() -> Self {
        Self {
            animals: HashMap::new()
        }
    }

    pub fn find_by_id(&self, id: i8) -> Option<&Animal> {
        return self.animals.get(&id);
    }

    pub fn find_all(&self) -> Vec<Animal> {
        return self.animals.values().cloned().collect::<Vec<Animal>>();
    }

    pub fn save(&mut self, animal: Animal) -> i8 {
        let new_id: i8 = self.animals.len() as i8 + 1;
        let new_animal = Animal {
            id: new_id,
            ..animal
        };
        self.animals.insert(new_id, new_animal);
        new_id
    }

    pub fn delete_by_id(&mut self, id: i8) -> Option<Animal> {
        return self.animals.remove(&id);
    }

    pub fn count(&self) -> i8 {
        return self.animals.len() as i8;
    }
}