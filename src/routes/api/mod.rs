use actix_web::{
    web
};

mod register;
mod authorization;

pub fn config_api(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/registration").configure(register::config))
        .service(web::resource("/login").route(web::post().to(authorization::login)))
        .service(web::resource("/logout").route(web::post().to(authorization::logout)));
}