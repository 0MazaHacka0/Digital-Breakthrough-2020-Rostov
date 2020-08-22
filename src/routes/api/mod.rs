use actix_web::{
    web
};

mod register;
mod authorization;

pub fn config_api(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/registration").configure(register::config));
    cfg.service(web::scope("/login").configure(authorization::config));
}