use actix_web::{web, App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::index)
            .service(web::scope("/api").service(routes::health))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
