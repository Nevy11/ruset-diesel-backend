use diesel::prelude::*;

use crate::schema::studentinfo;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::studentinfo)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StudentInfo {
    pub reg_no: String,
    pub first_name: String,
    pub last_name: String,
    pub grade: String,
    pub marks: i32,
}

#[derive(Insertable)]
#[diesel(table_name=studentinfo)]
pub struct NewStudentInfo<'a> {
    pub reg_no: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub grade: &'a str,
    pub marks: i32,
}
