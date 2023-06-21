use crate::dto::animal_request_dto::AnimalRequestDto;
use crate::dto::animal_response_dto::AnimalResponseDto;
use crate::model::animal::Animal;

impl From<&Animal> for AnimalResponseDto {
    fn from(animal: &Animal) -> Self {
        Self {
            id: animal.id(),
            species: animal.species().to_string(),
            common_name: animal.common_name().to_string(),
            habitat: animal.habitat().to_string(),
            lifespan: animal.lifespan(),
            is_endangered: animal.is_endangered(),
        }
    }
}

impl From<&AnimalRequestDto> for Animal {
    fn from(request_dto: &AnimalRequestDto) -> Self {
        Animal::new(
            -1,
            request_dto.species().to_string(),
            request_dto.common_name().to_string(),
            request_dto.habitat().to_string(),
            request_dto.lifespan(),
            request_dto.is_endangered(),
        )
    }
}
