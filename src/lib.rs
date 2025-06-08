//------import section-----------
pub mod models;
//-------------------------------

//---------main feature----------
mod handler;
use actix_web::{ App, HttpServer, dev::Server};
use std::net::TcpListener;
use std::sync::Arc;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let json_config_limit = handler::sample_json::json_config_limit()?;
    let server = HttpServer::new(move || {
        App::new()
            .app_data(json_config_limit.clone())
            .service(handler::sample_json::sample_json_api)
            .service(handler::health_check::health_check_api)
            .service(handler::subscribe::subscriptions_api)
    })
    .listen(listener)?
    .run();
    Ok(server)
}