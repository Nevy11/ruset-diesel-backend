// use std::env;

use diesel::{Connection, PgConnection};

pub fn establish_connection() -> PgConnection {
    // let database_url = env::var("database_url").expect("DATABASE_URL must be set");
    println!("Don't forget to add the database to the .env variable");
    let database_url = "postgres://nevy11:Skyworth.95@localhost/rust_backend_diesel".to_string();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
