use actix_web::{get, HttpResponse};

#[tracing::instrument(name = "Checking the application health")]
#[get("/")]
pub async fn hello_world() -> HttpResponse {
    println!("hello world");
    HttpResponse::Ok().body("hi there")
}