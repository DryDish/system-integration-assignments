use std::{env, fs::File, io::BufReader};

pub fn buffer_from_file(file_name: &str) -> BufReader<File> {
    let current_dir = env::current_dir().expect("Failed to get PWD.");
    let files_path = current_dir.join("../files/");

    let file_result = File::open(files_path.join(file_name));

    match file_result {
        Ok(file) => {
            return BufReader::new(file);
        }
        Err(error) => {
            panic!("FILE NOT FOUND!\n{}", error);
        }
    }
}
