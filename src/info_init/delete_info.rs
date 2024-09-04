use actix_web::{delete, HttpResponse, Responder};

#[delete("/info")]
pub async fn delete_info() -> impl Responder {
    HttpResponse::Ok().body("Delete's a student's info")
}
