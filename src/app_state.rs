use crate::app_config::AppConfig;
use crate::db_util::establish_connection;

use crate::repository::db_animal_repository::DbAnimalRepository;
use crate::service::animal_service::AnimalService;

pub struct AppState {
    pub animal_service: AnimalService,
}

impl AppState {
    pub fn new(app_config: &AppConfig) -> Self {
        let db_pool = establish_connection(&app_config.database_url);
        let animal_repository = DbAnimalRepository::new(db_pool.clone());

        Self {
            animal_service: AnimalService::new(animal_repository),
        }
    }
}
