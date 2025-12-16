#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GreetPerson {
    pub name: String,
    pub age: u8,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GreetPersonResp {
    pub message: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreatePerson {
    name: 

    
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreatePersonResp {}
