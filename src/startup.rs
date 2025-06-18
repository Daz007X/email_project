use actix_web::{ App, HttpServer, dev::Server, web};
use actix_web::middleware::Logger;

use std::net::TcpListener;
use sqlx::PgPool;
use crate::handler;


pub fn run(listener: TcpListener,db_pool: PgPool) -> Result<Server, std::io::Error> {
    let json_config_limit = handler::sample_json::json_config_limit()?;
     let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(json_config_limit.clone())
            .app_data(db_pool.clone())
            .service(handler::sample_json::sample_json_api)
            .service(handler::health_check::health_check_api)
            .service(handler::subscribe::subscriptions_api)
    })
    .listen(listener)?
    .run();
    Ok(server)
}