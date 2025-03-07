use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server =
        HttpServer::new(|| App::new().route("/health_check", web::get().to(HttpResponse::Ok)))
            .listen(listener)?
            .run();

    Ok(server)
}
