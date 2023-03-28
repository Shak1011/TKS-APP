use actix_web::{get, HttpResponse, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| actix_web::App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello world!</h1>")
}
