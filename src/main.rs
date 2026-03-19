use actix_web::dev::Server;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    run(listener)?.await
}
