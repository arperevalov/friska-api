use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[get("/lists")]
async fn lists_get() -> impl Responder {
    HttpResponse::Ok().body("Hello lists!")
}

#[get("/cards")]
async fn cards_get() -> impl Responder {
    HttpResponse::Ok().body("Hello cards!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init();

    let db_url = std::env::var("DB_URL").expect("DB_URL must be set");
    let db_username = std::env::var("DB_USERNAME").expect("DB_USERNAME must be set");
    let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME must be set");
    let db_connect_url = vec!["postgresql://", &db_username, ":", &db_password, "@", &db_url, "/", &db_name].join("");

    println!("{:?}",db_connect_url);
    let pool = match PgPoolOptions::new().max_connections(10).connect(&db_connect_url).await {
        Ok(pool) => {
            println!("{:?}", pool);
        }
        Err(error) => {
            println!("{error}")
        }
    };

    HttpServer::new(|| {
        App::new()
            .service(lists_get)
            .service(cards_get)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}