mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| actix_web::App::new().service(routes::index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
