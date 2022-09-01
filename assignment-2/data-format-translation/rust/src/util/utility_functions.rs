use std::{env, path::PathBuf};

pub fn env_as_num(env: &str) -> Option<i32> {
    let env = env::var(env).map(|var| var.parse::<i32>().unwrap());

    match env {
        Ok(env) => Some(env),
        Err(_) => None,
    }
}

#[allow(dead_code)]
pub fn env_as_str(env: &str) -> Option<String> {
    let env = env::var(env);

    match env {
        Ok(env) => Some(env),
        Err(_) => None,
    }
}

pub fn get_file_path(file_name: &str) -> PathBuf {
    let current_dir = env::current_dir().expect("Failed to get PWD.");
    let files_path = current_dir.join("../files/");
    files_path.join(file_name)
}
