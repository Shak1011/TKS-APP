#![allow(unused_imports)]

use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool, Connection};
use actix_files as fs;
use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use sqlx::sqlite::{SqlitePoolOptions,SqliteQueryResult, SqliteRow};

mod routes;

#[derive(Clone)]
pub struct AppState {
    pool: SqlitePool,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //For reference of this database setup use this website: https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/
    const DB_URL: &str = "sqlite://Users.db";

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    // Created database pool. If it breaks any additional changes made to the database delete it and I will adjust my changes.
    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(DB_URL)
        .await
        .unwrap();

    let app_state = AppState { pool };


    //Code beneath is used to insert data into the database. Uncomment if you want to insert the data or the table.

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS users (user_id INTEGER PRIMARY KEY NOT NULL,
        name VARCHAR(250) NOT NULL,
        email VARCHAR(250) NOT NULL,
        HashedPassword VARCHAR(250) NOT NULL,
        Elo INTEGER NOT NULL,
        Age INTEGER);
        
        CREATE TABLE IF NOT EXISTS profile_img (user_id INTEGER PRIMARY KEY NOT NULL,
        path VARCHAR(250) NOT NULL,
        CONSTRAINT fk_users
            FOREIGN KEY (user_id)
            REFERENCES users(user_id)
        );")
        .execute(&db).await.unwrap();
    println!("Create user table result: {:?}", result);
    
    let _result = sqlx::query("INSERT INTO users (name,email,HashedPassword,Elo,Age) VALUES ($1,$2,$3,$4,$5)")
    .bind("shak").bind("hscq37@durham.ac.uk").bind("HASHEDPASSWORD").bind(1000).bind(19)
    .execute(&db)
    .await
    .unwrap();



    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone())) //uses the database pool
            .service(fs::Files::new("/client","../client").show_files_listing())
            .service(routes::index)
            .service(
                web::scope("/api")
                .service(routes::health)
                .service(routes::get_all_users)
                .service(routes::get_user)
                .service(routes::get_all_users_elo)
                .service(routes::update_users_elo)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
