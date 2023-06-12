use std::ops::DerefMut;
use std::sync::Arc;
use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::model::animal::Animal;
use crate::repository::animal_repository::AnimalRepository;
use crate::schemas::schema::animals::dsl::animals;
use crate::schemas::schema::animals::id;

#[derive(Clone)]
pub struct DbAnimalRepository {
    db_pool: Pool<ConnectionManager<PgConnection>>
}

impl DbAnimalRepository {
    pub(crate) fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            db_pool
        }
    }

    fn get_connection(&self) -> &mut PgConnection {
        self.db_pool.get().unwrap().deref_mut()
    }
}

impl AnimalRepository for DbAnimalRepository {
    fn find_by_id(&self, animal_id: i32) -> Option<Animal> {
        return animals
            .filter(id.eq(animal_id))
            .first::<Animal>(self.get_connection())
            .ok()
    }

    fn find_all(&self) -> Vec<Animal> {
        return animals.load::<Animal>(self.get_connection()).unwrap();
    }

    fn save(&self, animal: Animal) -> i32 {
        let db_conn = self.get_connection();
        let new_id: i32 = animals.count().first::<i32>(db_conn) + 1;

        let new_animal = Animal::new(new_id, animal.species().to_string(),
                                     animal.common_name().to_string(), animal.habitat().to_string(),
                                     animal.lifespan(), animal.is_endangered());

        diesel::insert_into(animals::table)
            .values(new_animal)
            .returning(Animal::as_returning())
            .get_result(db_conn)
            .expect("Error saving new post");

        new_id
    }

    fn delete_by_id(&self, animal_id: i32) -> Option<Animal> {
        diesel::delete(animals.filter(id.eq(animal_id)))
            .execute(self.get_connection());
    }

    fn count(&self) -> i32 {
        return animals.count().first::<i32>(self.get_connection()).unwrap();
    }
}
