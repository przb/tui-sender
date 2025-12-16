use crate::app::people_info::{Person, Pet};

#[derive(
    Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize, thiserror::Error,
)]
pub enum ResponseError {
    #[error("missing fields")]
    MissingFields,

    #[error("field too big")]
    FieldTooBig,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GreetPerson {
    pub name: String,
    pub age: u8,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GreetPersonResp {
    pub message: Result<String, ResponseError>,
}

#[expect(unused)]
#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreatePerson {
    name: String,
    birth_day: u8,
    birth_month: u8,
    birth_year: u16,
    weight: u16,
    pets: Vec<Pet>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[expect(unused)]
pub struct CreatePersonResp {
    person: Result<Person, ResponseError>,
}
