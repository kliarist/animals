use diesel::dsl::{count_star};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::model::animal::Animal;
use crate::repository::animal_repository::AnimalRepository;
use crate::schemas::schema::animals::dsl::animals;
use crate::schemas::schema::animals::{id, table};

#[derive(Clone)]
pub struct DbAnimalRepository {
    db_pool: Pool<ConnectionManager<PgConnection>>
}

impl DbAnimalRepository {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            db_pool
        }
    }
}

impl AnimalRepository for DbAnimalRepository {
    fn find_by_id(&self, animal_id: i32) -> Option<Animal> {
        let db_conn = &mut self.db_pool.get().unwrap();
        return animals
            .filter(id.eq(animal_id))
            .first::<Animal>(db_conn)
            .ok()
    }

    fn find_all(&self) -> Vec<Animal> {
        let db_conn = &mut self.db_pool.get().unwrap();
        return animals.load::<Animal>(db_conn).unwrap();
    }

    fn save(&self, animal: Animal) -> i32 {
        let db_conn = &mut self.db_pool.get().unwrap();
        let i = self.count() as i32;
        let new_id: i32 = i + 1;

        let new_animal = Animal::new(new_id, animal.species().to_string(),
                                     animal.common_name().to_string(), animal.habitat().to_string(),
                                     animal.lifespan(), animal.is_endangered());

        diesel::insert_into(table)
            .values(new_animal)
            .returning(Animal::as_returning())
            .get_result(db_conn)
            .expect("Error saving new post");

        new_id
    }

    fn delete_by_id(&self, animal_id: i32) -> Option<Animal> {
        let db_conn = &mut self.db_pool.get().unwrap();
        let animal = animals
            .filter(id.eq(animal_id))
            .first::<Animal>(db_conn)
            .ok();

        let _ = diesel::delete(animals.filter(id.eq(animal_id)))
            .execute(db_conn);
        animal
    }

    fn count(&self) -> i64 {
        let db_conn = &mut self.db_pool.get().unwrap();
        animals.select(count_star()).first::<i64>(db_conn).unwrap()
    }
}
