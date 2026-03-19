use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn health_check(req: Option<HttpRequest>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    async fn health_check_succeeds() {
        let response = health_check(None).await;
        assert!(response.status().is_success())
    }
}
