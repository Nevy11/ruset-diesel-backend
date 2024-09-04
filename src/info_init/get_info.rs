use actix_web::{get, HttpResponse, Responder};

use crate::operations::crud_operations::show_students_info::show_students_info;

#[get("/info")]
pub async fn get_info() -> impl Responder {
    let student_info = show_students_info();
    HttpResponse::Ok().body(format!("{:?}", student_info))
}
