mod control;
mod models;
mod repl;
mod schema;
use diesel::prelude::*;
use dotenvy::dotenv;
use repl::repl_loop;
use std::env;

fn main() {
    repl_loop(1, &mut establish_connection());
}
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
