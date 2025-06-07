use std::net::TcpListener;


pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = email_project::run(listener).expect("Failed to start server");
    tokio::spawn(server);
    //tracing::info!("Port run at: {}", &port);
    format!("http://{}:{}", "127.0.0.1", port)
}