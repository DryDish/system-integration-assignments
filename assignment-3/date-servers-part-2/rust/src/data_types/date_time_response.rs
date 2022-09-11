use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct DateTimeResponse {
    #[schema(example = "2022-09-11T17:14:15.527468+00:00")]
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
