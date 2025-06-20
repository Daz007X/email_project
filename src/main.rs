use email_project::configuration::get_configuration;
use email_project::startup::run;
use std::net::TcpListener;
use sqlx::PgPool;
use env_logger::Env;
use std::env;



#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
     println!("RUST_LOG: {:?}", env::var("RUST_LOG"));
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // สร้าง Port และ Connection string DB 
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(
            &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");
   

    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    run(listener,connection_pool)?.await

}
