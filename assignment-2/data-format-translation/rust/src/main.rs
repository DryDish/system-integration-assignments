mod data_types;
mod route_handlers;
mod util;

use actix_web::{middleware, web, App, HttpServer};
use route_handlers::{json_route, not_found_route, xml_route, yaml_route};
use util::utility_functions::{env_as_num, get_file_path};

extern crate dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up
    dotenv::from_path(get_file_path(".env")).expect("Unable to load .env file!");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let server_port = env_as_num("RUST_PORT").expect("RUST_PORT env not found!") as u16;

    // Run Server and apply service routes
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async { "Page not found!" }))
            .service(json_route)
            .service(yaml_route)
            .service(xml_route)
            .default_service(web::route().to(not_found_route))
    })
    .bind(("127.0.0.1", server_port))?
    .run()
    .await
}

// Run `cargo test -- --nocapture` to get the println's to show
#[cfg(test)]
mod tests {
    use crate::{
        data_types::user::{Address, User},
        util::buffer_parser::{parse_json, parse_xml, parse_yaml},
    };

    #[test]
    fn print_test() {
        // Create test user
        let test_user: User = User::new(
            "NAME TEST",
            99,
            Address::new("TEST STREET NAME", "TEST FLOOR", 4321),
            &["TEST LIKE", "TEST LIKE 2", "TEST LIKE 3"],
        );

        // Print it for visual validation
        println!("{:#?}", test_user);

        // Minimal assertion to let test pass
        assert!(test_user.name == "NAME TEST");
    }

    #[test]
    fn print_test_two() {
        let json_user_result = parse_json("user.json");
        let yaml_user_result = parse_yaml("user.yaml");
        let xml_user_result = parse_xml("user.xml");

        match &json_user_result {
            Ok(user) => println!("Json User: \n{:#?}", user),
            Err(error) => println!("Failed to parse Json: {:?}", error),
        }

        match &yaml_user_result {
            Ok(user) => println!("Yaml User: \n{:#?}", user),
            Err(error) => println!("Failed to parse Yaml: {:?}", error),
        }

        match &xml_user_result {
            Ok(user) => println!("Xml User: \n{:#?}", user),
            Err(error) => println!("Failed to parse Xml: {:?}", error),
        }

        assert!(1 == 1);

        assert!(json_user_result.is_ok());
        assert!(yaml_user_result.is_ok());
        assert!(xml_user_result.is_ok());
    }
}
