use crate::model::animal::Animal;


pub trait AnimalRepository {
    fn find_by_id(&self, id: i64) -> Option<Animal>;

    fn find_all(&self) -> Vec<Animal>;

    fn save(&self, animal: Animal) -> i64;

    fn delete_by_id(&self, id: i64) -> Option<Animal>;

    fn count(&self) -> i64;
}
