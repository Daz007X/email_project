use actix_web::{web, 
                App, 
                //HttpRequest, 
                HttpServer, 
                //Responder,
                //HttpResponse,
                dev::Server};

use std::net::TcpListener;
mod handler;

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(handler::health_check::health_check_api)
            //.route("/health_check", web::get().to(handler::health_check::health_check_api))
    })
    .listen(listener)?
    .run();
    Ok(server)
}