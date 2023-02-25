use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().json("Hello World!!")
}
