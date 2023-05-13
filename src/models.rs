use diesel::prelude::*;

#[derive(Queryable)]
pub struct Todo {
    pub id: String,
    pub description: String,
    pub completed: bool,
}
