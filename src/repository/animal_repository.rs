use crate::animal::Animal;
use crate::model::animal::Animal;

pub trait AnimalRepository {
    fn find_by_id(&self, id: i8) -> Option<&Animal>;

    fn find_all(&self) -> Vec<Animal>;

    fn save(&mut self, animal: Animal) -> i8;

    fn delete_by_id(&mut self, id: i8) -> Option<Animal>;

    fn count(&self) -> i8;
}