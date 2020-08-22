use crate::schema::{companys, users};
use serde::Deserialize;

#[derive(Queryable, Clone)]
pub struct Route {
    pub id: i32,
    pub name: String,
    pub route: String,
    pub description: Option<String>,
    pub publication: i16
}

#[derive(Queryable)]
pub struct Page {
    pub id: i32,
    pub route_name: String,
    pub page_name: String,
    pub description: Option<String>,
    pub path: String
}

#[derive(Queryable)]
pub struct Static {
    pub id: i32,
    pub page_id: i32,
    pub name: Option<String>,
    pub type_file: String,
    pub status: i16,
    pub mask: String
}

#[derive(Queryable)]
pub struct Region {
    pub id: i32,
    pub name: String
}

#[derive(Queryable, Insertable, Deserialize)]
#[table_name = "companys"]
pub struct Company {
    pub id: i32,
    pub password_hash: String,
    pub email: String,
    pub region_id: i32
}

#[derive(Queryable, Insertable, Deserialize)]
#[table_name = "companys"]
pub struct CompanyOmitId {
    pub password_hash: String,
    pub email: String,
    pub region_id: i32
}

#[derive(Queryable)]
pub struct Houms {
    pub id: i32,
    pub company_id: i32,
    pub description: Option<String>,
}

#[derive(Queryable, Insertable, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub password_hash: String,
    pub email: String,
    pub region_id: i32,
    pub home_id: i32,
}

#[derive(Queryable, Insertable, Deserialize)]
#[table_name = "users"]
pub struct UserOmitId {
    pub password_hash: String,
    pub email: String,
    pub region_id: i32,
    pub home_id: i32,
}
