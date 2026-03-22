use actix_web::web::{self, Form};
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError, http::StatusCode};
use chrono::Utc;
use sqlx::PgPool;
use std::fmt;
use std::io::Error;
use std::ops::Sub;
use uuid::Uuid;

///creating a custom data type is like this
/// first we wrap what we want to give out into a struct as we dont own it and then
/// provide it the functionalities from another trait we dont own.
/// in this case we use sqlx:Error for wrapping and use stuff from actix_web::ResponseError to give
/// functionality.
#[derive(Debug)]
pub struct SubscribeError(sqlx::Error);

//then we implement the fmt::Display type for our newly created struct
impl fmt::Display for SubscribeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "A database failure was encountered while trying to store a subscriber."
        )
    }
}

//then we define the ResponseError trait for our struct and its functions status code and error response
impl ResponseError for SubscribeError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).body("Internal Server Error")
    }
}

//now to implement ? we need to implement the `From` trait so we do that From<sqlx::Error> on the
// SubscribeError struct
impl From<sqlx::Error> for SubscribeError {
    fn from(value: sqlx::Error) -> Self {
        Self(value)
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>,
) -> Result<HttpResponse, SubscribeError> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id,email, name, subscribed_at)
        VALUES ($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}
