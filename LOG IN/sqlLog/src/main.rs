use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://Users.db";

#[tokio::main]
async fn main() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL,
        name VARCHAR(250) NOT NULL,
        email VARCHAR(250) NOT NULL,
        HashedPassword VARCHAR(250) NOT NULL,
        Elo INTEGER NOT NULL);")
        .execute(&db).await.unwrap();
    println!("Create user table result: {:?}", result);

    let _result = sqlx::query("INSERT INTO users (name,email,HashedPassword,Elo) VALUES ($1,$2,$3,$4)")
    .bind("shak").bind("hscq37@durham.ac.uk").bind("HASHEDPASSWORD").bind(1000)
    .execute(&db)
    .await
    .unwrap();
}
