use std::env::args;
use std::io::stdin;

use crate::{
    connection_psql::establishing_connection::establish_connection, models::StudentInfo,
    schema::studentinfo::dsl::studentinfo,
};

use diesel::prelude::*;
use diesel::SelectableHelper;

pub fn get_students_info() {
    let mut data = String::new();
    println!("Enter the reg number to fetch the data");
    stdin().read_line(&mut data).unwrap();

    let data = data.trim();

    // let post_id = args().nth(1).expect("get_posts requires a post id");
    // println!("post_id: {}", post_id);
    let connection = &mut establish_connection();
    let post = studentinfo
        .find(&data)
        .select(StudentInfo::as_select())
        .first(connection)
        .optional();

    match post {
        Ok(Some(info)) => println!(
            "Info with id: {}, is called: {}",
            info.reg_no, info.first_name
        ),
        Ok(None) => println!("failed find reg no {}", data),
        Err(_) => println!("An error occured while fetching info: {}", data),
    }
}
