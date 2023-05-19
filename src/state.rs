use crate::animal_service::AnimalService;
use crate::service::animal_service::AnimalService;

pub struct AppState {
    pub animal_service: AnimalService
}

impl AppState {
    pub fn new() -> Self {
        let animal_repository = Box::new(AnimalRepositoryImpl::new());
        Self {
            animal_service: AnimalService::new(animal_repository)
        }
    }
}