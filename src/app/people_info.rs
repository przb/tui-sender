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

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Pet {
    pub weight: Kilograms,
    pub age: u8,
    pub typ: PetType,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PetType {
    Cat { lives: u8 },
    Dog { has_floppy_ears: bool },
    Hamster,
    Elephant,
}
