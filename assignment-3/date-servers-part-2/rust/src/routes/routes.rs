use crate::data_types;
use chrono;

use actix_web::{get, HttpResponse, Responder};
use data_types::date_time_response::DateTimeResponse;
use data_types::http_response::CustomHttpResponse;

/// Route that returns the current timestamp in UTC time following the 
/// ISO-8601 standard.
#[get("/datetime")]
async fn date_time_route() -> impl Responder {
    let utc = chrono::Utc::now();

    let res = DateTimeResponse {
        utc_time_stamp: utc.to_rfc3339(),
    };
    HttpResponse::Ok().json(res)
}

/// Route that calls another api at: `localhost:3000/datetime`, parses the response and
/// returns it to the caller.
#[get("/datetime_from_python")]
async fn date_time_from_python_route() -> impl Responder {
    let res = reqwest::get("http://localhost:3000/datetime").await;

    match res {
        Ok(time) => {
            let response = time.json::<DateTimeResponse>().await.unwrap();
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::InternalServerError().json(CustomHttpResponse::internal_server_error())
        }
    }
}

/// Default route to return a Json body with `"status": 404` and `"message": "Not Found."`.
pub async fn not_found_route() -> HttpResponse {
    HttpResponse::NotFound().json(CustomHttpResponse::not_found())
}
