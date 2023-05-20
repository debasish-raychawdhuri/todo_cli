mod control;
mod models;
mod repl;
mod schema;
use diesel::prelude::*;
use dotenvy::dotenv;
use repl::login;
use std::env;

fn main() {
    let mut conn = establish_connection();
    login(&mut conn).unwrap();
}
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
