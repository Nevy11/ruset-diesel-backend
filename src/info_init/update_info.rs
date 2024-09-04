use actix_web::{patch, HttpResponse, Responder};

#[patch("/info")]
pub async fn update_info() -> impl Responder {
    HttpResponse::Ok().body("Updates the student's info")
}
