use crate::dto::animal_request_dto::AnimalRequestDto;
use crate::dto::animal_response_dto::AnimalResponseDto;
use crate::model::animal::Animal;
use crate::repository::animal_repository::AnimalRepository;
use crate::repository::db_animal_repository::DbAnimalRepository;

pub struct AnimalService {
    repository: DbAnimalRepository,
}

impl AnimalService {
    pub fn new(repository: DbAnimalRepository) -> Self {
        Self { repository }
    }

    pub fn find_by_id(&self, id: i32) -> Option<AnimalResponseDto> {
        self.repository
            .find_by_id(id)
            .map(|animal| AnimalResponseDto::from(&animal))
    }

    pub fn find_all(&self) -> Vec<AnimalResponseDto> {
        self.repository
            .find_all()
            .iter()
            .map(AnimalResponseDto::from)
            .collect()
    }

    pub fn save(&mut self, dto: AnimalRequestDto) -> i32 {
        let animal = Animal::from(&dto);
        self.repository.save(animal)
    }

    pub fn delete_by_id(&mut self, id: i32) -> Option<AnimalResponseDto> {
        self.repository
            .delete_by_id(id)
            .map(|animal| AnimalResponseDto::from(&animal))
    }
}
