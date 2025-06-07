use email_project::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    //ปั้น port 
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("port:{}",port);

    //server run 
    run(listener).await?.await;
    
}