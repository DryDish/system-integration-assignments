use crate::data_types;
use crate::util;

use actix_web::{Responder, HttpResponse, get};
use util::buffer_parser::{parse_json, parse_xml, parse_yaml};
use data_types::http_response::ResponseError;

#[get("/json")]
async fn json_route() -> impl Responder {
    let json_user_result = parse_json("user.json");
    HttpResponse::Ok().json(json_user_result.unwrap())
}

#[get("yaml")]
async fn yaml_route() -> impl Responder {
    let yaml_user_result = parse_yaml("user.yaml");
    HttpResponse::Ok().json(yaml_user_result.unwrap())
}

#[get("/xml")]
async fn xml_route() -> impl Responder {
    let xml_user_result = parse_xml("user.xml");
    HttpResponse::Ok().json(xml_user_result.unwrap())
}

pub async fn not_found_route() -> HttpResponse {
    HttpResponse::NotFound().json(ResponseError::not_found())
}
