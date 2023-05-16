use crate::schema::todos;
use crate::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub user_id: i32,
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
