use diesel::prelude::*;

use crate::{connection_psql::establishing_connection::establish_connection, models::StudentInfo};

pub fn show_students_info() -> Vec<StudentInfo> {
    use crate::schema::studentinfo::dsl::*;
    let connection = &mut establish_connection();
    let results = studentinfo
        // .limit(5)
        .select(StudentInfo::as_select())
        .load(connection)
        .expect("Error loading posts");
    results
}
