use actix_web::{get, HttpResponse};

use crate::service::hello::hello_world;

#[tracing::instrument(name = "Checking the application health")]
#[get("/")]
pub async fn hello_world_handler() -> HttpResponse {
    let res = hello_world();
    HttpResponse::Ok().body(res)
}