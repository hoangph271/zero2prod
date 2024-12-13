use zero2prod;

pub fn spawn_app() {
    let server = zero2prod::run("127.0.0.1:8000").expect("Failed to start zero2prod::run");

    let _ = tokio::spawn(server);
}
