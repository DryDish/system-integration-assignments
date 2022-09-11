use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DateTimeResponse {
    pub utc_time_stamp: String,
}

impl DateTimeResponse {
    #[allow(dead_code)]
    pub fn new<T: ToString>(time_stamp: T) -> Self {
        DateTimeResponse {
            utc_time_stamp: time_stamp.to_string(),
        }
    }
}
