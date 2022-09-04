use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseError {
    pub error: u16,
    pub message: String,
}

impl ResponseError {
    pub fn new<T: ToString>(error: u16, message: T) -> Self {
        ResponseError {
            error,
            message: message.to_string(),
        }
    }

    pub fn not_found() -> Self {
        ResponseError {
            error: 404,
            message: String::from("Not found!"),
        }
    }
}
