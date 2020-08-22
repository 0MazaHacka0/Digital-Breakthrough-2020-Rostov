use actix_web::{
    web,
    HttpResponse,
    Result,
    Error,
    http
};

use serde::{Deserialize, Serialize};

use crate::models::database::{Company, User};
use crate::models::api::NewAccount;

use crate::database::read::companys;
use crate::database::read::users;

use crate::database::create::company as create_company;
use crate::database::create::user as create_user;

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}

#[derive(Deserialize, Serialize, Debug)]
struct Answer {
    code: i32,
    message: String
}

async fn register_user(
    params: web::Json<NewAccount>
) -> Result<web::Json<Answer>, Error> {
    if users::check_account(params.email.to_owned()) {
        create_user::save(params.into_inner());

        let answer = Answer { code: 200, message: "ok".to_owned() };

        return Ok(web::Json(answer))
    }

    let answer = Answer { code: 500, message: "User with such mail already exists".to_owned() };

    Ok(web::Json(answer))
}

async fn register_company(
    params: web::Json<NewAccount>
) -> Result<web::Json<Answer>, Error> {
    if companys::check_account(params.email.to_owned()) {
        create_company::save(params.into_inner());

        let answer = Answer { code: 200, message: "ok".to_owned() };

        return Ok(web::Json(answer))
    }

    let answer = Answer { code: 500, message: "User with such mail already exists".to_owned() };

    Ok(web::Json(answer))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("").route(web::post().to(register_user)))
        .service(web::resource("/").route(web::post().to(register_user)))
        .service(web::resource("/user").route(web::post().to(register_user)))
        .service(web::resource("/company").route(web::post().to(register_company)));
}