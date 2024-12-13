use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run("0.0.0.0:8000")?.await
}
