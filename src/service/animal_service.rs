use crate::dto::animal_request_dto::AnimalRequestDto;
use crate::dto::animal_response_dto::AnimalResponseDto;
use crate::model::animal::Animal;
use crate::repository::animal_repository::AnimalRepository;

pub struct AnimalService {
    repository: Box<dyn AnimalRepository + Send + Sync>,
}

impl AnimalService {
    pub fn new(repository: Box<dyn AnimalRepository + Send + Sync>) -> Self {
        Self { repository }
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
        let animal = Animal::from(&dto);
        return self.repository.save(animal);
    }

    pub fn delete_by_id(&mut self, id: i8) -> Option<AnimalResponseDto> {
        return self.repository
            .delete_by_id(id)
            .map(|animal| { AnimalResponseDto::from(&animal) });
    }
}
