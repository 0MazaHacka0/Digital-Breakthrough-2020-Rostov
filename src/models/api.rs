use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct NewAccount {
    pub phone: String,
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
    pub phone: String,
    pub password: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Vote {
    pub home_id: i32,
    pub description: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct VoteIds {
    pub home_id: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Votes {
    pub vote_id: i32,
}