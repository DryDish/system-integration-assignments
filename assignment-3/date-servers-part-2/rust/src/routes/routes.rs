use crate::data_types;
use chrono;

use actix_web::{get, HttpResponse, Responder};
use data_types::date_time_response::DateTimeResponse;
use data_types::http_response::CustomHttpResponse;

/// Returns the current timestamp in UTC time following the ISO-8601 standard.
#[utoipa::path(
    responses(
        (status = 200,
            description = "The current UTC Datetime formatted in the ISO-8601 standard.",
            body = DateTimeResponse,
            example = json!({ "utc_time_stamp": "2022-09-11T17:14:15.527468+00:00"})),
        (status = 500,
            description = "Unexpected error has occurred.",
            body = HttpResponse,
            example = json!({"status": 500, "message": "Internal Server Error"}))
    )
)]
#[get("/datetime")]
async fn date_time_route() -> impl Responder {
    let utc = chrono::Utc::now();

    let res = DateTimeResponse {
        utc_time_stamp: utc.to_rfc3339(),
    };
    HttpResponse::Ok().json(res)
}

/// Returns the current UTC time in ISO-8601 format from another python server.
///
/// Sends a request to `localhost:3000/datetime`, parses the response and then
/// responds to the caller with the datetime that was received.
#[utoipa::path(
    responses(
        (status = 200,
            description = "Datetime successfully retrieved from python server", 
            body = DateTimeResponse),
        (status = 500,
            description = "Unexpected error has occurred", 
            body = CustomHttpResponse,
            example = json!({"status": 500, "message": "Internal Server Error"}))
    )
)]
#[get("/datetime_from_python")]
async fn date_time_from_python_route() -> impl Responder {
    let res = reqwest::get("http://localhost:3000/datetime").await;

    match res {
        Ok(time) => {
            let response = time.json::<DateTimeResponse>().await.unwrap();
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            println!("Error was: {:?}", err);
            HttpResponse::InternalServerError().json(CustomHttpResponse::internal_server_error())
        }
    }
}

/// Default route
///
/// Returns a Json body with `"status": 404` and `"message": "Not Found."`.
/// This is the default response for all undefined routes.
#[utoipa::path(
    get,
    path = "/*",
    responses(
        (status = 404,
            description = "Page Not Found.",
            body= CustomHttpResponse,
            example = json!({"status": 404, "message": "Not Found"}))
        )
)]
pub async fn not_found_route() -> HttpResponse {
    HttpResponse::NotFound().json(CustomHttpResponse::not_found())
}
