use actix_web::{get, HttpResponse, Responder};

#[get("/info/{uuid}")]
pub async fn get_one(uuid: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Gets {} data", uuid))
}
