use actix_web::{http, web, HttpResponse};

use crate::database::read::routes::get_all_routes;
use crate::database::read::pages::get_page;
use tera::Tera;

mod app;
mod api;
pub mod error;

lazy_static! {
    pub static ref TEMPLATE: Tera = {
        let tera = match Tera::new("pub/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                error!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        tera
    };
}

fn redirect_to(location: &str) -> HttpResponse {
    info!("REDIRECT TO: {}", location);
    
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}

pub fn get_template_path(route_name: &str, alias_page: &str) -> String {
    let page = get_page(route_name, alias_page);

    format!(
        "{route_name}/templates/{path}",
        route_name=page.route_name,
        path=page.path
    )
}

pub fn get_page_path(route_name: &str, alias_page: &str) -> String {
    let page = get_page(route_name, alias_page);

    format!(
        "./pub/{route_name}/templates/{path}",
        route_name=page.route_name,
        path=page.path
    )
}

pub fn config_server(cfg: &mut web::ServiceConfig) {
    let routes = get_all_routes();

    if let Some(error_route) = routes.clone().into_iter().find(|item| item.name == "error") {
        cfg.service(web::scope(&error_route.route).configure(error::config_error));
    } else {
        warn!("Error routes not active...");
    }

    if let Some(app_route) = routes.clone().into_iter().find(|item| item.name == "app") {
        let route = app_route.route;

        cfg.service(web::scope(&route).configure(app::config_app));
        cfg.route("/", web::get().to(move || redirect_to(&route)));
    } else {
        warn!("App routes not active...");
    }

    if let Some(api_route) = routes.clone().into_iter().find(|item| item.name == "api") {
        let route = api_route.route;

        info!("{}", route.to_owned());

        cfg.service(web::scope(&route).configure(api::config_api));
    } else {
        warn!("Api routes not active...");
    }
}