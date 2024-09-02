use crate::{connection_psql::establishing_connection::establish_connection, models::StudentInfo};
use diesel::prelude::*;

pub fn update_students_info() {
    use crate::schema::studentinfo::dsl::{reg_no, studentinfo};
    // let's update the reg number
    let connection = &mut establish_connection();
    let id = "4\n";
    let post = diesel::update(studentinfo.find(id))
        .set(reg_no.eq("10"))
        .returning(StudentInfo::as_returning())
        .get_result(connection)
        .unwrap();
    println!("updated the reg no to {}", post.reg_no);
}
