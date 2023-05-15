use crate::dto::{AnimalRequestDto, AnimalResponseDto};
use crate::model::Animal;
use crate::repository::AnimalRepository;

#[derive(Clone)]
pub struct AnimalService {
    repository: AnimalRepository,
}

impl AnimalService {
    pub fn new() -> Self {
        Self {
            repository: AnimalRepository::new()
        }
    }

    pub fn find_by_id(&self, id: i8) -> Option<AnimalResponseDto> {
        return self.repository
            .find_by_id(id)
            .map(|animal| { AnimalResponseDto::from(animal) });
    }

    pub fn find_all(&self) -> Vec<AnimalResponseDto> {
        return self.repository.find_all().iter()
            .map(|animal| AnimalResponseDto::from(animal))
            .collect();
    }

    pub fn save(&mut self, dto: AnimalRequestDto) -> i8 {
        let mut animal = Animal::from(&dto);
        let new_id: i8 = self.repository.count() + 1;
        animal.id = new_id;
        self.repository.save(animal);
        new_id
    }

    pub fn delete_by_id(&mut self, id: i8) -> Option<AnimalResponseDto> {
        return self.repository
            .delete_by_id(id)
            .map(|animal| { AnimalResponseDto::from(&animal) });
    }
}