use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use routes::health_check::health_check;
use routes::subscribe::subscribe;
use std::net::TcpListener;
pub mod configuration;
pub mod routes;
use sqlx::PgPool;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new({
        move || {
            App::new()
                .route("/health_check", web::get().to(health_check))
                .route("/subscription", web::post().to(subscribe))
                .app_data(connection.clone())
        }
    })
    .listen(listener)?
    .run();

    Ok(server)
}
