use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserBody {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPublic {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPasswordBody {
    pub password: String,
}