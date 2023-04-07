use actix_files::NamedFile;
use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
pub async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("../index.html")
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({"alive": "true"}))
}
