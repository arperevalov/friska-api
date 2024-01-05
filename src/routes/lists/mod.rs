use crate::models::auth::Claims;
use crate::models::lists::{ListBody, List};
use crate::AppState;
use actix_web::{get, post, put, delete, HttpResponse, Responder, web, HttpRequest, HttpMessage};


#[get("/lists")]
pub async fn lists_get(req: HttpRequest, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = req.extensions().get::<Claims>().unwrap().id;

    let result = sqlx::query_as!(List, "SELECT * from lists where user_id = $1",
    user_id)
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[get("/lists/{id}")]
pub async fn lists_get_with_id(req: HttpRequest, path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = req.extensions().get::<Claims>().unwrap().id;
    let id = path.into_inner();

    let result = sqlx::query_as!(List, "SELECT * from lists where id=$1 and user_id = $2", id, user_id)
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[post("/lists")]
pub async fn lists_post(req: HttpRequest, body: web::Json<ListBody>, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = req.extensions().get::<Claims>().unwrap().id;

    let result = sqlx::query_as!(List, "INSERT into lists(title, user_id, best_before) values($1, $2, $3) returning *", 
    body.title.to_string(), 
    user_id,
    body.best_before)
    .fetch_one(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[put("/lists/{id}")]
pub async fn lists_put(req: HttpRequest, path: web::Path<i32>, body: web::Json<ListBody>, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = req.extensions().get::<Claims>().unwrap().id;
    let id = path.into_inner();

    let result = sqlx::query_as!(List, "UPDATE lists set title = $1, user_id = $2, best_before = $3 where id = $4 returning *", 
    body.title.to_string(), 
    user_id, 
    body.best_before,
    id)
    .fetch_one(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[delete("/lists/{id}")]
pub async fn lists_delete(req: HttpRequest, path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let user_id = req.extensions().get::<Claims>().unwrap().id;

    let _result = sqlx::query_as!(List, "DELETE from lists where id = $1 and user_id = $2", id, user_id)
    .fetch_optional(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok()
}