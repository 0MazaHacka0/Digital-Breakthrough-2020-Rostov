use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewAccount {
    pub email: String,
    pub password: String,
    pub home_id: Option<i32>
}
