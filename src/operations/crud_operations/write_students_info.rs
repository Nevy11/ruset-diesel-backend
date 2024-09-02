use std::io::{stdin, Read};

use crate::{
    connection_psql::establishing_connection::establish_connection,
    operations::create_post::create_post,
};

pub fn write_students_info() {
    let connection = &mut establish_connection();
    let mut update_reg_no = String::new();
    println!("update reg number");
    stdin().read_line(&mut update_reg_no).unwrap();
    let update_reg_no = update_reg_no.trim_end();
    // let new_update = update_reg_no;
    let reg_no = update_reg_no;
    let first_name = "Stephen";
    let last_name = "Mainda";
    let grade = "B";
    let marks = 56;

    let registered_data = create_post(connection, reg_no, first_name, last_name, grade, marks);
    println!("\nSaved draft {:?} with id {}", registered_data, reg_no);
}
