use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, database};
use anyhow::{Result};
use sha2::{Sha256, Digest};
const DB_URL: &str = "sqlite://Users.db";
#[no_mangle]
//SHA 2 IMPLEMENTATION, except rather than a file it is a single string which is password
pub fn hash(password: &str) -> Result<[u8; 32]> {
    let mut hasher = Sha256::new();
    hasher.update(&password);
    let hash= hasher.finalize();
    let mut ret: [u8; 32] = <[u8; 32]>::default();
    ret.copy_from_slice(&hash);
    return Ok(ret)
}
pub fn hash_to_string(data: &[u8]) -> String{
    let mut hashedpassword = String::new();
    for i in data {
        let x = format!("{:02x}", i);
        hashedpassword.push_str(&x);
    }
    hashedpassword
}
pub extern fn structure(password: &str){
    let mut return_to_node = String::new();
    if let Ok(hash) = hash(password){
    println!("{}", hash_to_string(&hash));
    return_to_node = hash_to_string(&hash)

}} 
pub extern fn add_user(user_info: &[u3]) {
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let _result = sqlx::query("INSERT INTO users (name,email,HashedPassword,Elo) VALUES ($1,$2,$3,$4)")
    .bind(user_info[0]).bind(user_info[1]).bind(structure(user_info[2])).bind(1000)
    .execute(&db)
    .await
    .unwrap();
}

pub extern fn comfirm_user(user_info: &[u2]){
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let _result = sqlx::query("SELECT row FROM users WHERE email = $1 AND HASHEDPASSWORD = $2")
    .bind(user_info[0]).bind(structure(user_info[1]))
    .execute(&db).await.unwrap();
    return _result
}