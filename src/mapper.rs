use crate::dto::{AnimalRequestDto, AnimalResponseDto};
use crate::model::Animal;

impl From<&Animal> for AnimalResponseDto {
    fn from(animal: &Animal) -> Self {
        Self {
            id: animal.id,
            age: animal.age,
            kind: String::from(&animal.kind),
            sound: String::from(&animal.sound),
        }
    }
}

impl From<&AnimalRequestDto> for Animal {
    fn from(request_dto: &AnimalRequestDto) -> Self {
        Self {
            id: 0,
            age: request_dto.age,
            kind: String::from(&request_dto.kind),
            sound: String::from(&request_dto.sound),
        }
    }
}
