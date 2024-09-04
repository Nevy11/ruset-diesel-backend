use actix_web::{App, HttpServer};
use info_init::{
    delete_info::delete_info, get_info::get_info, post_info::post_info, update_info::update_info,
};

mod connection_psql;
pub mod info_init;
pub mod models;
pub mod operations;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_info)
            .service(post_info)
            .service(update_info)
            .service(delete_info)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
