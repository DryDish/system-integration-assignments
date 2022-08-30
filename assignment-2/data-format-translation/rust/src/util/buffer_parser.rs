use crate::data_types::user::User;

use super::buffer_generator::buffer_from_file;

pub fn parse_json(file_name: &str) -> Result<User, serde_json::Error> {
    let json_file_buffer = buffer_from_file(file_name);
    serde_json::from_reader(json_file_buffer)
}

pub fn parse_yaml(file_name: &str) -> Result<User, serde_yaml::Error> {
    let yaml_file_buffer = buffer_from_file(file_name);
    serde_yaml::from_reader(yaml_file_buffer)
}

pub fn parse_xml(file_name: &str) -> Result<User, quick_xml::de::DeError> {
    let xml_file_buffer = buffer_from_file(file_name);
    quick_xml::de::from_reader(xml_file_buffer)
}
