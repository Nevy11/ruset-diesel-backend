use crate::connection_psql::establishing_connection::establish_connection;
use diesel::prelude::*;

pub fn delete_students_info() {
    use crate::schema::studentinfo::dsl::*;
    let target = "1";
    let pattern = format!("%{}%", target);
    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(studentinfo.filter(reg_no.like(pattern)))
        .execute(connection)
        .expect("Error deleting student's info");
    println!("Deleted {} info", num_deleted);
}
