use actix_web::{web,post, Responder,HttpResponse,error};
use crate::models::sample_json::Info; 


#[post("/sample_json")]
pub async fn sample_json_api(_info: web::Json<Info>) ->impl Responder 
{
    HttpResponse::Ok().body("JSON received successfully")
}

pub fn json_config_limit() -> Result<web::JsonConfig, std::io::Error> {
    let json_config = web::JsonConfig::default()
        .limit(128)
        .error_handler(|err, _req| {
            println!("Hit error handler: {:?}", err); 
            error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
        });

    Ok(json_config)
}