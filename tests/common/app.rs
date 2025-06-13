use std::net::TcpListener;
use email_project::startup::run;
use sqlx::{PgConnection, Connection};


pub fn spawn_app(connection: PgConnection) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener,connection).expect("Failed to start server");
    tokio::spawn(server);
    //tracing::info!("Port run at: {}", &port);
    format!("http://{}:{}", "127.0.0.1", port)
}