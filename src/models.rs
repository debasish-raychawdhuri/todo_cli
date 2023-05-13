use crate::schema::todos;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
pub struct Todo {
    pub id: String,
    pub description: String,
    pub completed: bool,
}
