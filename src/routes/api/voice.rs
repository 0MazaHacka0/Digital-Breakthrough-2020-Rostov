use actix_web::{
    web,
    Result,
    Error
};

use run_script::ScriptOptions;

use crate::models::api::{Vote, Answer, VoteIds, Votes};

async fn create_vote(
    data: web::Json<Vote>
)
-> Result<web::Json<Answer>, Error> {
    let options = ScriptOptions::new();
    let args = vec![];

    let (code, output, error) = run_script::run(
        &format!(r#"
        npm run blockchain:create_vote -- --home_id={home_id} --description={description}
        "#, home_id=data.home_id, description="".to_owned()),
        &args,
        &options
    ).unwrap();

    info!("code: {}", code);
    info!("output: {}", output);
    error!("error: {}", error);

    if error.len() > 0 {
        return Ok(web::Json(Answer { code: 500, message: "Error create vote!".to_owned() }))
    } else {
        return Ok(web::Json(Answer { code: 200, message: "Ok".to_owned() }))
    }
}

async fn get_vote_ids(
    data: web::Path<VoteIds>
)
-> Result<web::Json<Answer>, Error> {
    let options = ScriptOptions::new();
    let args = vec![];

    let (code, output, error) = run_script::run(
        &format!(r#"
        npm run blockchain:create_vote -- --home_id={home_id}
        "#, home_id=data.home_id),
        &args,
        &options
    ).unwrap();

    info!("code: {}", code);
    info!("output: {}", output);
    error!("error: {}", error);

    if error.len() > 0 {
        return Ok(web::Json(Answer { code: 500, message: "Error getting vote ids!".to_owned() }))
    } else {
        return Ok(web::Json(Answer { code: 200, message: "Ok".to_owned() }))
    }
}

async fn get_vote(
    data: web::Path<Votes>
)
-> Result<web::Json<Answer>, Error> {
    let options = ScriptOptions::new();
    let args = vec![];

    let (code, output, error) = run_script::run(
        &format!(r#"
        npm run blockchain:get_vote -- --vote_id={vote_id}
        "#, vote_id=data.vote_id),
        &args,
        &options
    ).unwrap();

    info!("code: {}", code);
    info!("output: {}", output);
    error!("error: {}", error);

    if error.len() > 0 {
        return Ok(web::Json(Answer { code: 500, message: "Error getting vote ids!".to_owned() }))
    } else {
        return Ok(web::Json(Answer { code: 200, message: "Ok".to_owned() }))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/vote").route(web::post().to(create_vote)))
        .service(web::resource("/vote/{:id}").route(web::get().to(get_vote)))
        .service(web::resource("/vote/ids/{:id}").route(web::get().to(get_vote_ids)));
}