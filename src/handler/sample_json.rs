use actix_web::{web,post, Responder,HttpResponse,error};
use crate::models::sample_json::Info; 
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use tracing::Instrument;

#[post("/sample_json")]
pub async fn sample_json_api(info: web::Json<Info>,pool :web::Data<PgPool>) ->impl Responder 
{
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new sample_json_Info.",
        %request_id,
        info_name = %info.name,
        info_email = %info.email,
        info_message = %info.message
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!(
        "Saving new json_Info details in the database"
    );
    match sqlx::query!(
        r#"
        INSERT INTO Info (id,name, email, message, sendAt)
        VALUES ($1, $2, $3, $4,$5)
        "#,
        Uuid::new_v4(),
        info.name,
        info.email,
        info.message,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => 
            {
            HttpResponse::Created().body("JSON received successfully")
            },
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

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