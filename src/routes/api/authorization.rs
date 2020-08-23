use actix_web::{
    web,
    Result,
    Error
};

use actix_identity::Identity;

use crate::models::api::{Answer, Loggin};

use crate::database::read::users::get_user_id;

pub async fn login(
    params: web::Json<Loggin>,
    id: Identity
) -> Result<web::Json<Answer>, Error> {
    let user_id = get_user_id(params.email.clone());


    if let Some(_identifier) = id.identity() {
        return Ok(web::Json(Answer { code: 400, message: "User is logging".to_owned() }))
    } else {
        id.remember(format!("user_{}", user_id).to_owned());

        return Ok(web::Json(Answer { code: 200, message: "Ok".to_owned() }))
    }
}

pub async fn logout(
    id: Identity
) -> Result<web::Json<Answer>, Error> {
    id.forget(); 

    Ok(web::Json(Answer { code: 200, message: "Ok".to_owned() }))
}