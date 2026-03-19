use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use routes::health_check::health_check;
use std::net::TcpListener;
pub mod routes;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server =
        HttpServer::new({ || App::new().route("/health_check", web::get().to(health_check)) })
            .listen(listener)?
            .run();

    Ok(server)
}
