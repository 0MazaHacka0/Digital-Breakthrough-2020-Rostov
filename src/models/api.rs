use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NewAccount {
    pub email: String,
    pub password: String,
    pub home_id: Option<i32>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Answer {
    pub code: i32,
    pub message: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Loggin {
    pub email: String,
    pub password: String
}
