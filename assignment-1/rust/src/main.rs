mod data_types;
mod util;


use util::buffer_generator::buffer_from_file;
use crate::data_types::user::User;

fn main() {
    let json_file_buffer = buffer_from_file("user.json");
    let yaml_file_buffer = buffer_from_file("user.yaml");
    let xml_file_buffer = buffer_from_file("user.xml");

    let json_user: Result<User, serde_json::Error> = serde_json::from_reader(json_file_buffer);
    let yaml_user: Result<User, serde_yaml::Error> = serde_yaml::from_reader(yaml_file_buffer);
    let xml_user: Result<User, quick_xml::de::DeError> = quick_xml::de::from_reader(xml_file_buffer);

    
    match json_user {
        Ok(user) => println!("Json User: \n{:#?}", user),
        Err(error) => println!("Failed to parse Json: {:?}", error),
    }

    match yaml_user {
        Ok(user) => println!("Yaml User: \n{:#?}", user),
        Err(error) => println!("Failed to parse Yaml: {:?}", error),
    }

    match xml_user {
        Ok(user) => println!("Xml User: \n{:#?}", user),
        Err(error) => println!("Failed to parse Xml: {:?}", error),
    }

}

#[cfg(test)]
mod tests {
    use crate::data_types::user::Address;

    use super::*;

    #[test]
    fn print_test() {
        // Create test user
        let test_user: User = User::new(
            "NAME TEST",
            99,
            Address::new("TEST STREET NAME", "TEST FLOOR", 4321),
            &["TEST LIKE", "TEST LIKE 2", "TEST LIKE 3"],
        );

        // Print it for visual validation
        println!("{:#?}", test_user);

        // Minimal assertion to let test pass
        assert!(test_user.name == "NAME TEST");
    }
}
