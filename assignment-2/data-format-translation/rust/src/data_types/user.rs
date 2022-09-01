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
    pub fn new<T: ToString>(name: &str, age: u8, address: Address, likes: &[T]) -> User {
        User {
            name: name.to_string(),
            age,
            address,
            likes: likes
                .iter()
                .map(|thing| thing.to_string())
                .collect(),
        }
    }
}

impl Address {
    pub fn new<T: ToString>(street_name: T, floor: T, area_code: u16) -> Address {
        Address {
            street_name: street_name.to_string(),
            floor: floor.to_string(),
            area_code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{User, Address};
 

    #[test]
    fn create_user_and_address() {

        let address = Address::new("street name", "first", 1234);
        let user = User::new("Peter", 12, address, &["likes", "some", "things"]);

        assert!(user.address.street_name == "street name");
        assert!(user.age == 12);
        assert!(user.name == "Peter");
        assert!(user.likes == vec!["likes".to_string(), "some".to_string(), "things".to_string()])
    }

    #[test]
    fn create_user_and_address_vector() {       
        let address = Address::new("street name", "first", 1234);
        let user = User::new("Peter", 12, address, &vec!("likes", "some", "things"));
        
        assert!(user.address.street_name == "street name");
        assert!(user.age == 12);
        assert!(user.name == "Peter");
        assert!(user.likes == vec!["likes".to_string(), "some".to_string(), "things".to_string()])
    }


}