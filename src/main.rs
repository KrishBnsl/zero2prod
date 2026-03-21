use actix_web::{HttpResponse, HttpServer, Responder};
use zero2prod::configuration::{Settings, get_configuration};
use zero2prod::{configuration, run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address).expect("failed to bind random port");
    run(listener)?.await
}
