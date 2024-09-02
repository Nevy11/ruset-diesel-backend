use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

use crate::{models::StudentInfo, schema::studentinfo};

pub fn create_post(
    conn: &mut PgConnection,
    reg_no: &str,
    first_name: &str,
    last_name: &str,
    grade: &str,
    marks: i32,
) -> StudentInfo {
    let new_data = StudentInfo {
        reg_no: reg_no.to_string(),
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        grade: grade.to_string(),
        marks: marks,
    };
    diesel::insert_into(studentinfo::table)
        .values(&new_data)
        .returning(StudentInfo::as_returning())
        .get_result(conn)
        .expect("Error saving new data")
}
