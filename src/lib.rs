//------import section-----------
pub mod models;
//-------------------------------

//---------main feature----------
mod handler;
use actix_web::{//web, 
                App, 
                //HttpRequest, 
                HttpServer, 
                //Responder,
                //HttpResponse,
                dev::Server};
use std::net::TcpListener;
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(handler::health_check::health_check_api)
            .service(handler::subscribe::subscriptions_api)
    })
    .listen(listener)?
    .run();
    Ok(server)
}