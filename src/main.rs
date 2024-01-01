mod routes;
mod models;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use routes::lists::{lists_delete, lists_get, lists_get_with_id, lists_post, lists_put};
use routes::users::{users_delete, users_get, users_post, users_put};
use routes::cards::{cards_delete, cards_get, cards_get_with_id, cards_post, cards_put};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let app_port: u16 = std::env::var("APP_PORT").expect("APP_PORT must be set").parse().unwrap();

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
        let cors = Cors::default()
        .allowed_origin("http://localhost:3000")
        .allow_any_method()
        .allow_any_header()
        .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(actix_web::web::Data::new(AppState{db: pool.clone()}))
            .service(lists_get)
            .service(lists_get_with_id)
            .service(lists_post)
            .service(lists_put)
            .service(lists_delete)
            .service(cards_get)
            .service(cards_post)
            .service(cards_put)
            .service(cards_get_with_id)
            .service(cards_delete)
            .service(users_get)
            .service(users_post)
            .service(users_put)
            .service(users_delete)
    })
    .bind(("127.0.0.1", app_port))?
    .run()
    .await
}