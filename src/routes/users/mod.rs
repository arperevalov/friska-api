use crate::models::auth::Claims;
use crate::models::users::{User, UserBody, UserPublic};
use crate::AppState;
use actix_web::{get, post, put, delete, HttpResponse, Responder, web, HttpRequest, HttpMessage};

#[get("/users")]
pub async fn users_get(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(UserPublic, "SELECT id, username, email, created_at from users")
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[get("/users/current")]
pub async fn users_current_get(req: HttpRequest, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = req.extensions().get::<Claims>().unwrap().id;

    let result = sqlx::query_as!(UserPublic, "SELECT id, username, email, created_at from users where id = $1", user_id)
    .fetch_one(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[post("/users")]
pub async fn users_post(body: web::Json<UserBody>, app_state: web::Data<AppState>) -> impl Responder {
    let password_hash = body.password.to_string();

    let result = sqlx::query_as!(User, "INSERT into users(username, email, password_hash) values($1, $2, $3) returning *", 
        body.username.to_string(),
        body.email.to_string(),
        password_hash
    )
    .fetch_one(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[put("/users/{id}")]
pub async fn users_put(path: web::Path<i32>, body: web::Json<UserBody>, app_state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let password_hash = body.password.to_string();

    let result = sqlx::query_as!(User, "UPDATE users set username = $1, email = $2, password_hash = $3 where id = $4 returning *", 
        body.username.to_string(),
        body.email.to_string(),
        password_hash,
        id
    )
    .fetch_one(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[delete("/users/{id}")]
pub async fn users_delete(path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();

    let _result = sqlx::query_as!(User, "DELETE from users where id = $1", id)
    .fetch_optional(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok()
}