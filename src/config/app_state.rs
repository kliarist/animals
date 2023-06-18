use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

use crate::repository::db_animal_repository::DbAnimalRepository;
use crate::service::animal_service::AnimalService;

pub struct AppState {
    pub animal_service: AnimalService,
}

impl AppState {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {

        let animal_repository = DbAnimalRepository::new(db_pool);

        Self {
            animal_service: AnimalService::new(animal_repository),
        }
    }
}
