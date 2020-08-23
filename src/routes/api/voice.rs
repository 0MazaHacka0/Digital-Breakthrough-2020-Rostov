use actix_web::{
    web,
    Result,
    Error
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("").route(web::post().to(register_user)))
        .service(web::resource("/").route(web::post().to(register_user)))
        .service(web::resource("/user").route(web::post().to(register_user)))
        .service(web::resource("/company").route(web::post().to(register_company)));
}