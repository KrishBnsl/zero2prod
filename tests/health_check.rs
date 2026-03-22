use std::{fmt::format, mem::zeroed, net::TcpListener};

use crate::common::TestApp;
mod common;

#[tokio::test]
async fn health_check_works() {
    let app: TestApp = common::spawn_app().await;
    let client = reqwest::Client::new();
    println!("{}", &app.address);

    let response = client
        .get(&format!("http://{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
