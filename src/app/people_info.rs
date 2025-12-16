#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub birthdate: chrono::NaiveDate,
    pub pets: Vec<Pet>,
    pub weight: Kilograms,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Kilograms(u16);
impl Kilograms {
    pub fn new(weight: u16) -> Self {
        Self(weight)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Pet {
    pub name: String,
    pub weight: Kilograms,
    pub age: u8,
    pub pet_type: PetType,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PetType {
    Cat { lives: u8 },
    Dog { has_floppy_ears: bool },
    Hamster,
    Elephant,
}
