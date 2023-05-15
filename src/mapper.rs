use crate::dto::{AnimalRequestDto, AnimalResponseDto};
use crate::model::Animal;

impl From<&Animal> for AnimalResponseDto {
    fn from(animal: &Animal) -> Self {
        Self {
            id: animal.id,
            species: String::from(&animal.species),
            common_name: String::from(&animal.common_name),
            habitat: String::from(&animal.habitat),
            lifespan: animal.lifespan,
            is_endangered: animal.is_endangered
        }
    }
}

impl From<&AnimalRequestDto> for Animal {
    fn from(request_dto: &AnimalRequestDto) -> Self {
        Self {
            id: -1,
            species: String::from(&request_dto.species),
            common_name: String::from(&request_dto.common_name),
            habitat: String::from(&request_dto.habitat),
            lifespan: request_dto.lifespan,
            is_endangered: request_dto.is_endangered
        }
    }
}
