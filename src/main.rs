use actix_web::{web, App, HttpResponse, HttpServer};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(|| HttpResponse::Ok())))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
