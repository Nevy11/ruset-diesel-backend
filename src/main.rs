// use connection_psql::main_establishing_connection::main_establishing_connection;
mod connection_psql;
pub mod models;
mod operations;
pub mod schema;
use operations::{
    crud_operations::delete_students_info::delete_students_info, show_posts::show_posts,
};

fn main() {
    delete_students_info();
    show_posts();
    // write_students_info();
    // get_students_info();
    // update_students_info();
}
/*
Things to download
Chrono features = serde
cargo install diesel_cli --no-default-features --features postgres
*/
