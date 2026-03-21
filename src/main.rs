use actix_web::{HttpResponse, HttpServer, Responder};
use sqlx::{Connection, PgConnection};
use zero2prod::configuration::{Settings, get_configuration};
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("failed to read configuration");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address).expect("failed to bind random port");
    run(listener, connection)?.await
}
