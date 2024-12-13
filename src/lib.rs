use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

pub fn run(address: &str) -> Result<Server, std::io::Error> {
    let server =
        HttpServer::new(|| App::new().route("/health_check", web::get().to(|| HttpResponse::Ok())))
            .bind(address)?
            .run();

    Ok(server)
}
