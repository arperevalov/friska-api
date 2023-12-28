use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, types::chrono};
use serde::{Deserialize, Serialize};

struct AppState {
    db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize, Debug)]
struct List {
    id: i32,
    title: String,
    user_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Card {
    id: i32,
    title: String,
    exp_date: chrono::NaiveDateTime,
    left_count: i32,
    units: String,
    list_id: i32,
    user_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    username: String,
    email: String,
    password_hash: i32,
    created_at: chrono::NaiveDateTime,
}

#[get("/lists")]
async fn lists_get(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(List, "SELECT * from lists")
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[get("/cards")]
async fn cards_get(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(Card, "SELECT * from cards")
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[post("/cards")]
async fn cards_post(body: web::Json<Card>, app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(Card, "SELECT * from cards")
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match PgPoolOptions::new().max_connections(10).connect(&database_url).await {
        Ok(pool) => {
            println!("CONNECTED TO DATABASE");
            pool
        }
        Err(error) => {
            println!("{error}");
            std::process::exit(1)
        }
    };

    HttpServer::new(move || {
        
        App::new()
            .app_data(actix_web::web::Data::new(AppState{db: pool.clone()}))
            .service(lists_get)
            .service(cards_get)
            .service(cards_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}