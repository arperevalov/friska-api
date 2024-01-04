mod routes;
mod models;
mod hasher;

use actix_web::dev::ServiceRequest;
use actix_web::{web, HttpMessage};
use actix_web::{App, HttpServer, Error};
use actix_cors::Cors;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, self};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use routes::lists::{lists_delete, lists_get, lists_get_with_id, lists_post, lists_put};
use routes::users::{users_delete, users_get, users_post, users_put, users_current_get};
use routes::cards::{cards_delete, cards_get, cards_get_with_id, cards_post, cards_put, cards_increment_put, cards_decrement_put};
use routes::auth::{auth_sign_in_post, auth_sign_up_post};

use crate::models::auth::Claims;

pub struct AppState {
    db: Pool<Postgres>,
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let token_string = credentials.token();
    let validation = Validation::new(Algorithm::HS256);

    let token_message = decode::<Claims>(&token_string, &decoding_key, &validation);
    
    match token_message {
        Ok(value) => {
            req.extensions_mut().insert(value.claims);
            Ok(req)
        },
        Err(_) => {
            let config = req.app_data::<bearer::Config>().cloned().unwrap_or_default().scope("");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
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

        let bearer_middleware = HttpAuthentication::bearer(validator);

        App::new()
            .wrap(cors)
            .app_data(actix_web::web::Data::new(AppState{db: pool.clone()}))
            .service(auth_sign_in_post)
            .service(auth_sign_up_post)
            .service(
                web::scope("")
                .wrap(bearer_middleware)
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
                .service(cards_increment_put)
                .service(cards_decrement_put)
                .service(users_get)
                .service(users_post)
                .service(users_put)
                .service(users_delete)
                .service(users_current_get)
            )
    })
    .bind(("127.0.0.1", app_port))?
    .run()
    .await
}