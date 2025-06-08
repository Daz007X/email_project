use email_project::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    run(listener)?.await

}
