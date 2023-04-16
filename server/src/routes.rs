#![allow(non_snake_case)]

use actix_files::NamedFile;
use actix_web::{web::Redirect, get, post, web, App, HttpServer, HttpResponse, Responder};
use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool, Connection};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{AppState};

#[derive(FromRow, Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    HashedPassword: String,
    Elo: i32,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
struct Elo {
    name: String,
    Elo: i32,
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[get("/")]
pub async fn index() -> impl Responder {
    Redirect::to("http://127.0.0.1:8080/client/index.html").permanent()
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({"alive": "true"}))
}

#[get("/get/{id}")]
pub async fn get_user(path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder{
    let user_id: i32 = path.into_inner(); 
    let query = format!("SELECT * FROM users WHERE id = {}", user_id);
    let users: Vec<User> = sqlx::query_as::<_, User>(&query) //Note: honestly couldn't figure out why I could not use the query_as! macro. It something to do with database url tho.
    .fetch_all(&app_state.pool).await.unwrap();

    if users.is_empty() {
        return HttpResponse::BadRequest().json(Response {
            message: "No user found with given id.".to_string()
        });
    }

    HttpResponse::Ok().json(users)
}


#[get("/users")]
pub async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let users: Vec<User> = sqlx::query_as::<_, User>("SELECT * FROM users") //Note: honestly couldn't figure out why I could not use the query_as! macro. It something to do with database url tho.
    .fetch_all(&app_state.pool).await.unwrap();

    HttpResponse::Ok().json(users)
}


#[get("/elo")]
pub async fn get_all_users_elo(app_state: web::Data<AppState>) -> impl Responder {
    let elos: Vec<Elo> = sqlx::query_as::<_, Elo>("SELECT name, elo FROM users ORDER BY elo DESC") 
    .fetch_all(&app_state.pool).await.unwrap();

    HttpResponse::Ok().json(elos)
}