use config::Config;
use sqlx::{Connection, PgPool};
use std::net::TcpListener;

use zero2prod::configuration::{Settings, get_configuration};

pub async fn spawn_app() -> String {
    let configuration =
        get_configuration().expect("failed to get configuration as configuration file not found");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to postgres server");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener, connection).expect("failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
