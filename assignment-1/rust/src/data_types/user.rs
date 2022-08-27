use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub age: u8,
    pub address: Address,
    pub likes: Vec<String>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Address {
    pub street_name: String,
    pub floor: String,
    pub area_code: u16,
}



impl User {
    #[allow(dead_code)]
    pub fn new<T: AsRef<str>>(name: &str, age: u8, address: Address, likes: &[T]) -> User {
        User {
            name: name.to_string(),
            age,
            address,
            likes: likes.iter().map(|value| value.as_ref().to_string()).collect(),
        }
    }
}

impl Address {
    #[allow(dead_code)]
    pub fn new(street_name: &str, floor: &str, area_code: u16) -> Address {
        Address {
            street_name: street_name.to_string(),
            floor: floor.to_string(),
            area_code,
        }
    }
}
