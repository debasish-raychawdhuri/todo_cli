use std::error::Error;

use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
    TextExpressionMethods,
};

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
    old_password: &'a str,
    new_password: &'a str,
) -> Result<(), Box<dyn Error>> {
    use schema::users;

    let user = users::dsl::users
        .filter(users::dsl::password.eq(old_password))
        .filter(users::dsl::id.eq(user_id))
        .first::<User>(conn)?;

    diesel::update(users::dsl::users.find(user.id))
        .set(users::dsl::password.eq(new_password))
        .filter(users::dsl::id.eq(user_id).or(users::dsl::id.eq(1)))
        .execute(conn)?;
    Ok(())
}

pub fn create_new_todo(
    conn: &mut PgConnection,
    user_id: i32,
    description: &str,
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

pub fn get_username_for_user_id(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<String, Box<dyn Error>> {
    use schema::users;

    let user = users::dsl::users
        .filter(users::dsl::id.eq(user_id))
        .first::<User>(conn)?;

    Ok(user.username)
}

pub fn authenticate_user(
    conn: &mut PgConnection,
    username: &str,
    password: &str,
) -> Result<i32, Box<dyn Error>> {
    use schema::users;

    let user = users::dsl::users
        .filter(users::dsl::username.eq(username))
        .filter(users::dsl::password.eq(password))
        .first::<User>(conn);

    match user {
        Ok(user) => Ok(user.id),
        Err(_) => Err("Invalid username or password".into()),
    }
}

pub fn delete_todo(conn: &mut PgConnection, user_id: i32, id: i32) -> Result<(), Box<dyn Error>> {
    use schema::todos;

    let result = diesel::delete(todos::dsl::todos.find(id))
        .filter(todos::dsl::user_id.eq(user_id))
        .get_result::<Todo>(conn);

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Todo not found".into()),
    }
}

pub fn get_all_pending_todos_for_user(
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

pub fn get_all_todos_for_user(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<Vec<crate::models::Todo>, Box<dyn Error>> {
    use schema::todos;

    let results = todos::dsl::todos
        .filter(todos::dsl::user_id.eq(user_id))
        .load::<crate::models::Todo>(conn)?;

    Ok(results)
}

pub fn update_todo(
    conn: &mut PgConnection,
    user_id: i32,
    id: i32,
    description: &str,
) -> Result<(), Box<dyn Error>> {
    use schema::todos;

    let result = diesel::update(todos::dsl::todos.find(id))
        .set(todos::dsl::description.eq(description))
        .filter(todos::dsl::user_id.eq(user_id))
        .get_result::<Todo>(conn);

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Todo not found".into()),
    }
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

pub fn mark_todo_done(
    conn: &mut PgConnection,
    user_id: i32,
    id: i32,
) -> Result<(), Box<dyn Error>> {
    use schema::todos;

    let result = diesel::update(todos::dsl::todos.find(id))
        .set(todos::dsl::completed.eq(true))
        .filter(todos::dsl::user_id.eq(user_id))
        .get_result::<Todo>(conn);

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Todo not found".into()),
    }
}
