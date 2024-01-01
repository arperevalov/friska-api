use actix_web::{get, post, put, App, HttpResponse, HttpServer, Responder, web};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, types::chrono};
use serde::{Deserialize, Serialize};

struct AppState {
    db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListBody {
    title: String,
    user_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct List {
    id: i32,
    title: String,
    user_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CardBody {
    title: String,
    exp_date: String,
    left_count: i32,
    units: String,
    list_id: i32,
    user_id: i32,
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
struct UserBody {
    username: String,
    email: String,
    password_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserPublic {
    id: i32,
    username: String,
    email: String,
    created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    username: String,
    email: String,
    password_hash: String,
    created_at: chrono::NaiveDateTime,
}

#[get("/lists")]
async fn lists_get(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(List, "SELECT * from lists")
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[get("/lists/{id}")]
async fn lists_get_with_id(path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query_as!(List, "SELECT * from lists where id=$1", id)
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[post("/lists")]
async fn lists_post(body: web::Json<ListBody>, app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(List, "INSERT into lists(title, user_id) values($1, $2) returning *", body.title.to_string(), body.user_id)
    .fetch_one(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[put("/lists/{id}")]
async fn lists_put(path: web::Path<i32>, body: web::Json<ListBody>, app_state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query_as!(List, "UPDATE lists set title = $1, user_id = $2 where id= $3 returning *", body.title.to_string(), body.user_id, id)
    .fetch_one(&app_state.db)
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

#[get("/cards/{id}")]
async fn cards_get_with_id(path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query_as!(Card, "SELECT * from cards where id=$1", id)
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[post("/cards")]
async fn cards_post(body: web::Json<CardBody>, app_state: web::Data<AppState>) -> impl Responder {
    let date = chrono::NaiveDateTime::parse_from_str(body.exp_date.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();

    let result = sqlx::query_as!(Card, "INSERT into cards(title, exp_date, left_count, units, list_id, user_id) values($1, $2, $3, $4, $5, $6) returning *", 
        body.title.to_string(),
        date,
        body.left_count,
        body.units,
        body.list_id,
        body.user_id
    )
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[put("/cards/{id}")]
async fn cards_put(path: web::Path<i32>, body: web::Json<CardBody>, app_state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let date = chrono::NaiveDateTime::parse_from_str(body.exp_date.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();

    let result = sqlx::query_as!(Card, "UPDATE cards set title = $1, exp_date = $2, left_count = $3, units = $4, list_id = $5, user_id = $6 where id = $7 returning *", 
    body.title.to_string(),
    date,
    body.left_count,
    body.units,
    body.list_id,
    body.user_id, 
    id)
    .fetch_one(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[get("/users")]
async fn users_get(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(UserPublic, "SELECT id, username, email, created_at from users")
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[post("/users")]
async fn users_post(body: web::Json<UserBody>, app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(User, "INSERT into users(username, email, password_hash) values($1, $2, $3) returning *", 
        body.username.to_string(),
        body.email.to_string(),
        body.password_hash.to_string()
    )
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[put("/users/{id}")]
async fn users_put(path: web::Path<i32>, body: web::Json<UserBody>, app_state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();

    let result = sqlx::query_as!(User, "UPDATE users set username = $1, email = $2, password_hash = $3 where id = $4 returning *", 
        body.username.to_string(),
        body.email.to_string(),
        body.password_hash.to_string(),
        id
    )
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
            .service(lists_get_with_id)
            .service(lists_post)
            .service(lists_put)
            .service(cards_get)
            .service(cards_post)
            .service(cards_put)
            .service(cards_get_with_id)
            .service(users_get)
            .service(users_post)
            .service(users_put)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}