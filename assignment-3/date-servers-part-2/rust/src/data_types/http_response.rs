use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomHttpResponse {
    pub status: u16,
    pub message: String,
}


#[allow(dead_code)]
impl CustomHttpResponse {
    pub fn new<T: ToString>(code: u16, message: T) -> Self {
        CustomHttpResponse {
            status: code,
            message: message.to_string(),
        }
    }

    pub fn not_found() -> Self {
        CustomHttpResponse {
            status: 404,
            message: String::from("Not found."),
        }
    }

    pub fn internal_server_error() -> Self {
        CustomHttpResponse {
            status: 404,
            message: String::from("Internal Server Error."),
        }
    }
}
