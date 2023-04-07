use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello world!</h1>")
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({"alive": "true"}))
}
