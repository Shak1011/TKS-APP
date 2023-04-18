#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::result;

use actix_files::NamedFile;
use actix_web::{web::Redirect, get, post, web, App, HttpServer, HttpResponse, Responder};
use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool, Connection};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{AppState};

#[derive(FromRow, Debug, Serialize, Deserialize)]
struct User {
    user_id: i32,
    name: String,
    email: String,
    HashedPassword: String,
    Elo: i32,
    Age: i32,
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
pub async fn index() -> Result<NamedFile, std::io::Error>{
    NamedFile::open("../client/index.html")
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({"alive": "true"}))
}

#[get("/get/{id}")]
pub async fn get_user(path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder{
    let user_id: i32 = path.into_inner(); 
    let query = format!("SELECT * FROM users WHERE user_id = {}", user_id);
    println!("{}", query);
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


#[get("/new-elo/{userW}/{userL}")]
async fn update_users_elo(path: web::Path<(i32, i32)>, app_state: web::Data<AppState>) -> impl Responder {
    let (user1, user2) = path.into_inner();
    //I know there is a more efficient way to do this but I just want it to work.
    let eloUser1: Option<i32> = sqlx::query_scalar(
        "SELECT Elo FROM users WHERE user_id = $1",
    )
    .bind(&user1)
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    let eloUser2: Option<i32> = sqlx::query_scalar(
        "SELECT Elo FROM users WHERE user_id = $1",
    )
    .bind(&user2)
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    let eloUser1 = eloUser1.unwrap();

    let eloUser2 = eloUser2.unwrap();


    let (updated_elo_1, updated_elo_2) = elo_calc(eloUser1, eloUser2);

    let query = format!("UPDATE users SET Elo = {} WHERE user_id = {}", updated_elo_1, user1);
    let update_userW = sqlx::query(&query)
    .execute(&app_state.pool)
    .await
    .unwrap();

    let query2 = format!("UPDATE users SET Elo = {} WHERE user_id = {}", updated_elo_2, user2);
    let update_userL = sqlx::query(&query2)
    .execute(&app_state.pool)
    .await
    .unwrap();


    HttpResponse::Ok().json(json!({"message": "Users Elo have been updated"}))

}


fn elo_calc(a : i32, b: i32) -> (i32, i32) {
    let K = 32 as f32;

    let n = i32::pow(10, ((b-a)/400) as u32);
    let p = i32::pow(10, ((a-b)/400) as u32);

    let expected_score_A:f32 = 1.0 / (1.0 + (n as f32));
    let expected_score_B:f32 = 1.0 / (1.0 + (p as f32));

    
    let new_Elo_A = ((a as f32) + K*(1.0 - (expected_score_A ))).round() as i32;
    let new_Elo_B = ((b as f32) + K*(0.0 - (expected_score_B))).round() as i32;

    (new_Elo_A, new_Elo_B)

}