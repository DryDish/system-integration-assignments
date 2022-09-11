use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use chrono;
use data_types::http_response::ResponseError;
use serde::{Deserialize, Serialize};

mod data_types;

#[derive(Serialize, Deserialize, Debug)]
struct DateTimeResponse {
    date_time: String,
}

#[get("/datetime")]
async fn date_time_route() -> impl Responder {
    let utc = chrono::Utc::now();

    let res = DateTimeResponse {
        date_time: utc.to_rfc3339(),
    };
    HttpResponse::Ok().json(res)
}

pub async fn not_found_route() -> HttpResponse {
    HttpResponse::NotFound().json(ResponseError::not_found())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let server_port: u16 = 5000;

    // Run Server and apply service routes
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async { "Page not found!" }))
            .service(date_time_route)
            .default_service(web::route().to(not_found_route))
    })
    .bind(("127.0.0.1", server_port))?
    .run()
    .await
}
