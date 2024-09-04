use std::io::stdin;

use crate::{
    connection_psql::establishing_connection::establish_connection, models::StudentInfo,
    schema::studentinfo::dsl::studentinfo,
};

use diesel::prelude::*;
use diesel::SelectableHelper;

pub fn get_students_info() -> Option<StudentInfo> {
    let data = "4";

    let connection = &mut establish_connection();
    let post = studentinfo
        .find(&data)
        .select(StudentInfo::as_select())
        .first(connection)
        .optional();

    match post {
        Ok(Some(info)) => Some(info),
        Ok(None) => None,
        Err(err) => {
            panic!("failed loading post: {}", err);
        }
    }
}
