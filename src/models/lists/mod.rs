use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListBody {
    pub title: String,
    pub user_id: Option<i32>,
    pub best_before: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    pub id: i32,
    pub title: String,
    pub user_id: Option<i32>,
    pub best_before: i32,
}
