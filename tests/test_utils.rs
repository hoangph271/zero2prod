use std::net::TcpListener;
use zero2prod;

pub fn spawn_app() -> String {
    let address: TcpListener =
        TcpListener::bind("127.0.0.1:0").expect("Could not bind to random port");
    let port = address
        .local_addr()
        .expect("Could not get local address")
        .port();

    let server = zero2prod::run(address).expect("Failed to start zero2prod::run");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
