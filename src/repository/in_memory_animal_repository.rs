use std::collections::HashMap;

use crate::model::animal::Animal;
use crate::repository::animal_repository::AnimalRepository;

#[derive(Clone)]
pub struct InMemoryAnimalRepository {
    animals: HashMap<i8, Animal>,
}

impl InMemoryAnimalRepository {
    pub fn new() -> Self {
        Self {
            animals: HashMap::new()
        }
    }
}

impl AnimalRepository for InMemoryAnimalRepository {
    fn find_by_id(&self, id: i8) -> Option<&Animal> {
        return self.animals.get(&id);
    }

    fn find_all(&self) -> Vec<Animal> {
        return self.animals.values().cloned().collect::<Vec<Animal>>();
    }

    fn save(&mut self, animal: Animal) -> i8 {
        let new_id: i8 = self.animals.len() as i8 + 1;
        let new_animal = Animal::new(new_id, animal.species().to_string(),
                                     animal.common_name().to_string(), animal.habitat().to_string(),
                                     animal.lifespan(), animal.is_endangered());
        self.animals.insert(new_id, new_animal);
        new_id
    }

    fn delete_by_id(&mut self, id: i8) -> Option<Animal> {
        return self.animals.remove(&id);
    }

    fn count(&self) -> i8 {
        return self.animals.len() as i8;
    }
}
