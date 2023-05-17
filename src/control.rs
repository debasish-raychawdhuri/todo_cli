use std::error::Error;

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, TextExpressionMethods};

use crate::{
    models::{NewTodo, NewUser, Todo, User},
    schema,
};

pub fn create_new_user<'a>(
    conn: &mut PgConnection,
    user_id: i32,
    username: &'a str,
    password: &'a str,
) -> Result<User, Box<dyn Error>> {
    use schema::users;

    let new_user = NewUser { username, password };
    if user_id != 1 {
        return Err("You are not allowed to create a user with id 1".into());
    }
    Ok(diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)?)
}

pub fn change_user_password<'a>(
    conn: &mut PgConnection,
    user_id: i32,
    username: &'a str,
    password: &'a str,
) -> Result<User, Box<dyn Error>> {
    use schema::users;

    let user = users::dsl::users
        .filter(users::dsl::username.eq(username))
        .filter(users::dsl::id.eq(user_id))
        .first::<User>(conn)?;

    Ok(diesel::update(users::dsl::users.find(user.id))
        .set(users::dsl::password.eq(password))
        .get_result(conn)
        .expect("Error saving password"))
}

pub fn create_new_todo<'a>(
    conn: &mut PgConnection,
    user_id: i32,
    description: &'a str,
) -> Result<Todo, Box<dyn Error>> {
    use schema::todos;

    let new_todo = NewTodo {
        description,
        user_id,
    };

    let new_todo = diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(conn)?;

    Ok(new_todo)
}

pub fn get_all_pending_todos_for_user<'a>(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<Vec<crate::models::Todo>, Box<dyn Error>> {
    use schema::todos;

    let results = todos::dsl::todos
        .filter(todos::dsl::user_id.eq(user_id))
        .filter(todos::dsl::completed.eq(false))
        .load::<crate::models::Todo>(conn)?;

    Ok(results)
}

pub fn get_all_todos_for_user<'a>(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<Vec<crate::models::Todo>, Box<dyn Error>> {
    use schema::todos;

    let results = todos::dsl::todos
        .filter(todos::dsl::user_id.eq(user_id))
        .load::<crate::models::Todo>(conn)?;

    Ok(results)
}

pub fn update_todo<'a>(
    conn: &mut PgConnection,
    user_id: i32,
    id: i32,
    description: &'a str,
) -> Result<(), Box<dyn Error>> {
    use schema::todos;

    diesel::update(todos::dsl::todos.find(id))
        .set((todos::dsl::description.eq(description),))
        .filter(todos::dsl::user_id.eq(user_id))
        .execute(conn)?;

    Ok(())
}

pub fn search_todo_by_description(
    conn: &mut PgConnection,
    user_id: i32,
    description: &str,
) -> Result<Vec<crate::models::Todo>, Box<dyn Error>> {
    use schema::todos;

    let results = todos::dsl::todos
        .filter(todos::dsl::user_id.eq(user_id))
        .filter(todos::dsl::description.like(format!("%{}%", description)))
        .load::<crate::models::Todo>(conn)?;

    Ok(results)
}

pub fn mark_todo_done<'a>(
    conn: &mut PgConnection,
    user_id: i32,
    id: i32,
) -> Result<(), Box<dyn Error>> {
    use schema::todos;

    diesel::update(todos::dsl::todos.find(id))
        .set(todos::dsl::completed.eq(true))
        .filter(todos::dsl::user_id.eq(user_id))
        .execute(conn)?;

    Ok(())
}
