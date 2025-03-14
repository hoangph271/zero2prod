use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address: TcpListener = TcpListener::bind("127.0.0.1:8080")?;

    run(address)?.await
}
