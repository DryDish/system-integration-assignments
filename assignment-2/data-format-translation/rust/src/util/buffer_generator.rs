use std::{fs::File, io::BufReader};

use super::utility_functions::get_file_path;

pub fn buffer_from_file(file_name: &str) -> BufReader<File> {
    let file_result = File::open(get_file_path(file_name));

    match file_result {
        Ok(file) => {
            return BufReader::new(file);
        }
        Err(error) => {
            panic!("FILE NOT FOUND!\n{}", error);
        }
    }
}
