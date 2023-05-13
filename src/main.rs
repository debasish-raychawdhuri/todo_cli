mod models;
mod schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::Todo;
use schema::todos;
use std::env;

fn main() {
    let mut connection = establish_connection();
    let results = todos::table
        .filter(todos::completed.eq(false))
        .load::<models::Todo>(&mut connection)
        .expect("Error loading todos");
    for todo in results {
        println!("{}", todo.description);
        println!("{}", todo.completed);
    }
    let new_todo = Todo {
        id: String::from("1"),
        description: String::from("Learn Rust, learn fast"),
        completed: false,
    };
    diesel::insert_into(todos::table)
        .values(&new_todo)
        .on_conflict(todos::id)
        .do_update()
        .set((
            todos::completed.eq(true),
            todos::description.eq(&new_todo.description),
        ))
        .execute(&mut connection)
        .expect("Error saving new todo");
}
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
