use actix_web::{
    web,
    Result,
    Error
};

use run_script::ScriptOptions;

use crate::models::api::{NewAccount, Answer};

use crate::database::read::companys;
use crate::database::read::users;

use crate::database::create::company as create_company;
use crate::database::create::user as create_user;

fn save_user_in_bch(user_id: i32, home_id: i32) {
    let options = ScriptOptions::new();
    let args = vec![];

    let (code, output, error) = run_script::run(
        &format!(r#"
        npm run blockchain:add_user -- --user_id={user_id} --home_id={home_id}
        "#, user_id=user_id, home_id=home_id),
        &args,
        &options
    ).unwrap();

    info!("code: {}", code);
    info!("output: {}", output);
    error!("error: {}", error);
}

fn verify_new_user(account: NewAccount) -> bool {
    let check_account = users::check_account(account.phone.to_owned());
    
    if !check_account {
        return false;
    }

    return account.phone.len() == 11 && account.password.len() > 0
}

async fn register_user(
    params: web::Json<NewAccount>
) -> Result<web::Json<Answer>, Error> {
    let data = NewAccount {
        home_id: params.home_id,
        phone: params.phone.clone(),
        password: params.password.clone()
    };

    if verify_new_user(data) {   
        let id = create_user::save(params.into_inner());

        if id.0 == -1 {
            let answer = Answer { code: 500, message: "Error saving user".to_owned() };

            return Ok(web::Json(answer))
        }

        save_user_in_bch(id.0, id.1);

        let answer = Answer { code: 200, message: "ok".to_owned() };

        return Ok(web::Json(answer))
    }

    let answer = Answer { code: 500, message: "User with such mail already exists".to_owned() };

    Ok(web::Json(answer))
}

async fn register_company(
    params: web::Json<NewAccount>
) -> Result<web::Json<Answer>, Error> {
    if companys::check_account(params.phone.to_owned()) {
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