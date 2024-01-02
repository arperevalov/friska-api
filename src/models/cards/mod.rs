use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CardBody {
    pub title: String,
    pub exp_date: String,
    pub left_count: f64,
    pub units: String,
    pub list_id: i32,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub id: i32,
    pub title: String,
    pub exp_date: chrono::NaiveDateTime,
    pub left_count: f64,
    pub units: String,
    pub list_id: i32,
    pub user_id: i32,
}