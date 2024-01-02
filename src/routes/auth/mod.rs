use crate::models::auth::{CredentialsSignIn, CredentialsSignUp, Claims};
use crate::AppState;
use crate::models::users::User;
use actix_web::{post, HttpResponse, Responder, web};
use jsonwebtoken::{encode, EncodingKey, Algorithm, get_current_timestamp};

#[post("/auth/sign-in")]
async fn auth_sign_in_post(body: web::Json<CredentialsSignIn>, app_state: web::Data<AppState>) -> impl Responder {
    let query = sqlx::query_as!(User, "SELECT * from users where username = $1", body.username.clone())
    .fetch_one(&app_state.db).await;

    match query {
        Ok(value) => {
            if value.password_hash == body.password {
                let token = generate_token(value.id, value.username);

                HttpResponse::Ok().json(serde_json::json!({"token": token}))
            } else {
                HttpResponse::Forbidden().json(serde_json::json!({"error": "Wrong credentials"}))
            }
        }
        Err(_) => {
            HttpResponse::Forbidden().json(serde_json::json!({"error": "Wrong credentials"}))
        }
    }
}

#[post("/auth/sign-up")]
async fn auth_sign_up_post(body: web::Json<CredentialsSignUp>, app_state: web::Data<AppState>) -> impl Responder {
    let query = sqlx::query_as!(User, "INSERT into users(username, email, password_hash) values($1, $2, $3) returning *",
        body.username.to_string(),
        body.email.to_string(),
        body.password.to_string())
    .fetch_one(&app_state.db).await;

    match query {
        Ok(value) => {
            let token = generate_token(value.id, value.username);
            HttpResponse::Ok().json(serde_json::json!({"token": token}))
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": "Internal server error"}))
        }
    }
}

fn generate_token(id: i32, username: String) -> String {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let header = jsonwebtoken::Header::new(Algorithm::HS256);

    let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let data = Claims {
        id: id,
        username: username,
        exp: get_current_timestamp()
    };
    encode(&header, &data, &encoding_key).unwrap()
}