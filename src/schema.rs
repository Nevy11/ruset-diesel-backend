// @generated automatically by Diesel CLI.

diesel::table! {
    studentinfo (reg_no) {
        reg_no -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        grade -> Varchar,
        marks -> Int4,
    }
}
// reg_no: reg_no.to_string(),
//         first_name: first_name.to_string(),
//         last_name: last_name.to_string(),
//         grade: grade.to_string(),
//         marks: marks,
