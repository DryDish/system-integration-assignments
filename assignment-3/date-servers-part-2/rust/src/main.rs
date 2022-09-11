mod data_types;
mod routes;

use actix_web::{middleware, web, App, HttpServer};
use routes::routes::{date_time_route, date_time_from_python_route, not_found_route};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let server_port: u16 = 5000;

    // Run Server and apply service routes
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(date_time_route)
            .service(date_time_from_python_route)
            .default_service(web::route().to(not_found_route))
    })
    .bind(("127.0.0.1", server_port))?
    .run()
    .await
}
