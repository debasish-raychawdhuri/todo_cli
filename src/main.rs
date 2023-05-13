mod models;
mod schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

fn main() {
    let mut connection = establish_connection();
    let results = schema::todos::table
        .filter(schema::todos::completed.eq(false))
        .load::<models::Todo>(&mut connection)
        .expect("Error loading todos");
    for todo in results {
        println!("{}", todo.description);
        println!("{}", todo.completed);
    }
}
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
