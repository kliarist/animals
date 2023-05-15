use crate::service::AnimalService;

pub struct AppState {
    pub animal_service: AnimalService
}

impl AppState {
    pub fn new() -> Self {
        Self {
            animal_service: AnimalService::new()
        }
    }
}