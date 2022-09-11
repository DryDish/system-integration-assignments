mod data_types;
mod routes;

use crate::routes::routes::{date_time_from_python_route, date_time_route, not_found_route};
use actix_web::{middleware, web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::data_types::date_time_response::DateTimeResponse;
use crate::data_types::http_response::CustomHttpResponse;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let server_port: u16 = 5000;

    // Open Api generator config
    #[derive(OpenApi)]
    #[openapi(
        paths(
            routes::routes::date_time_from_python_route,
            routes::routes::date_time_route,
            routes::routes::not_found_route,
        ),
        components(
            schemas(DateTimeResponse, CustomHttpResponse)
        ),
        tags(
            (name="Rust Time Server", description="Time server written in rust 🚀.")
        )
    )]
    struct ApiDoc;

    // Run Server and apply service routes
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(date_time_route)
            .service(date_time_from_python_route)
            .service(SwaggerUi::new("/docs/{_:.*}").url("/swagger-ui.json", ApiDoc::openapi()))
            .default_service(web::route().to(not_found_route))
    })
    .bind(("127.0.0.1", server_port))?
    .run()
    .await
}
