use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};

use crate::{connection_psql::establishing_connection::establish_connection, models::StudentInfo};
// Get request
pub fn show_posts() {
    use crate::schema::studentinfo::dsl::*;
    let connection = &mut establish_connection();
    let results = studentinfo
        // .limit(5)
        .select(StudentInfo::as_select())
        .load(connection)
        .expect("Error loading posts");
    println!("Displaying {} posts", results.len());
    for student_information in results {
        println!("{}", student_information.reg_no);
        println!("----------------------\n");
        println!("{}", student_information.first_name);
        println!("----------------------\n");
        println!("{}", student_information.last_name);
        println!("----------------------\n");
        println!("{}", student_information.grade);
        println!("----------------------\n");
        println!("{}", student_information.marks);
        println!("----------------------\n");
        println!("----------------------\n");
        println!("----------------------\n");
    }
}
