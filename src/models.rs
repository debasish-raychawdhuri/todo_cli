use std::fmt::Display;

use crate::schema::todos;
use crate::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Debug)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub user_id: i32,
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.completed { "\u{2714}" } else { " " };
        write!(f, "{}: {} ({})", self.id, self.description, status)
    }
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub description: &'a str,
    pub user_id: i32,
}

#[derive(Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
