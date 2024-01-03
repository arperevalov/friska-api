use crate::models::auth::Claims;
use crate::models::cards::{Card, CardBody};
use crate::AppState;
use actix_web::{get, post, put, delete, HttpResponse, Responder, web, HttpRequest, HttpMessage};

#[get("/cards")]
async fn cards_get(req: HttpRequest, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = req.extensions().get::<Claims>().unwrap().id;

    let result = sqlx::query_as!(Card, "SELECT * from cards where user_id = $1", user_id)
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[get("/cards/{id}")]
async fn cards_get_with_id(req: HttpRequest, path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = req.extensions().get::<Claims>().unwrap().id;
    let id = path.into_inner();

    let result = sqlx::query_as!(Card, "SELECT * from cards where id=$1 and user_id = $2", id, user_id)
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[post("/cards")]
async fn cards_post(req: HttpRequest, body: web::Json<CardBody>, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = req.extensions().get::<Claims>().unwrap().id;
    let date = chrono::NaiveDateTime::parse_from_str(body.exp_date.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();

    let result = sqlx::query_as!(Card, "INSERT into cards(title, exp_date, left_count, units, list_id, user_id) values($1, $2, $3, $4, $5, $6) returning *", 
        body.title.to_string(),
        date,
        body.left_count,
        body.units,
        body.list_id,
        user_id
    )
    .fetch_all(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[put("/cards/{id}")]
async fn cards_put(req: HttpRequest, path: web::Path<i32>, body: web::Json<CardBody>, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = req.extensions().get::<Claims>().unwrap().id;
    let id = path.into_inner();
    let date = chrono::NaiveDateTime::parse_from_str(body.exp_date.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();

    let result = sqlx::query_as!(Card, "UPDATE cards set title = $1, exp_date = $2, left_count = $3, units = $4, list_id = $5, user_id = $6 where id = $7 returning *", 
    body.title.to_string(),
    date,
    body.left_count,
    body.units,
    body.list_id,
    user_id, 
    id)
    .fetch_one(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(result))
}

#[delete("/cards/{id}")]
async fn cards_delete(req: HttpRequest, path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = req.extensions().get::<Claims>().unwrap().id;
    let id = path.into_inner();

    let _result = sqlx::query_as!(Card, "DELETE from cards where id = $1 and user_id = $2", id, user_id)
    .fetch_optional(&app_state.db)
    .await.unwrap();

    HttpResponse::Ok()
}