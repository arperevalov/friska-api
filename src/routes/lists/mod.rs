use crate::models::lists::{ListBody, List};
use crate::AppState;
use actix_web::{get, post, put, delete, App, HttpResponse, HttpServer, Responder, web};


#[get("/lists")]
pub async fn lists_get(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(List, "SELECT * from lists")
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[get("/lists/{id}")]
pub async fn lists_get_with_id(path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query_as!(List, "SELECT * from lists where id=$1", id)
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[post("/lists")]
pub async fn lists_post(body: web::Json<ListBody>, app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(List, "INSERT into lists(title, user_id) values($1, $2) returning *", body.title.to_string(), body.user_id)
    .fetch_one(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[put("/lists/{id}")]
pub async fn lists_put(path: web::Path<i32>, body: web::Json<ListBody>, app_state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query_as!(List, "UPDATE lists set title = $1, user_id = $2 where id= $3 returning *", body.title.to_string(), body.user_id, id)
    .fetch_one(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[delete("/lists/{id}")]
pub async fn lists_delete(path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();

    let _result = sqlx::query_as!(List, "DELETE from lists where id = $1", id)
    .fetch_optional(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok()
}