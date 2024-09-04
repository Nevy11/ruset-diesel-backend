use actix_web::{post, HttpResponse, Responder};

#[post("/info")]
pub async fn post_info() -> impl Responder {
    HttpResponse::Ok().body("Adds a new student")
}
