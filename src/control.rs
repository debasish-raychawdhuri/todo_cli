use std::error::Error;

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::{
    models::{NewTodo, NewUser, User},
    schema,
};

fn create_new_user<'a>(
    conn: &mut PgConnection,
    username: &'a str,
    password: &'a str,
) -> Result<User, Box<dyn Error>> {
    use schema::users;

    let new_user = NewUser { username, password };

    Ok(diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user"))
}

fn change_user_password<'a>(
    conn: &mut PgConnection,
    username: &'a str,
    password: &'a str,
) -> Result<User, Box<dyn Error>> {
    use schema::users;

    let user = users::dsl::users
        .filter(users::dsl::username.eq(username))
        .first::<User>(conn)
        .expect("Error loading user");

    Ok(diesel::update(users::dsl::users.find(user.id))
        .set(users::dsl::password.eq(password))
        .get_result(conn)
        .expect("Error saving password"))
}

fn create_new_todo<'a>(
    conn: &mut PgConnection,
    description: &'a str,
    user_id: i32,
) -> Result<(), Box<dyn Error>> {
    use schema::todos;

    let new_todo = NewTodo {
        description,
        user_id,
    };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(conn)
        .expect("Error saving new todo");

    Ok(())
}

fn get_all_pending_todos_for_user<'a>(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<Vec<crate::models::Todo>, Box<dyn Error>> {
    use schema::todos;

    let results = todos::dsl::todos
        .filter(todos::dsl::user_id.eq(user_id))
        .filter(todos::dsl::completed.eq(false))
        .load::<crate::models::Todo>(conn)
        .expect("Error loading todos");

    Ok(results)
}

fn update_todo<'a>(
    conn: &mut PgConnection,
    id: i32,
    description: &'a str,
    completed: bool,
) -> Result<(), Box<dyn Error>> {
    use schema::todos;

    diesel::update(todos::dsl::todos.find(id))
        .set((
            todos::dsl::description.eq(description),
            todos::dsl::completed.eq(completed),
        ))
        .execute(conn)
        .expect("Error saving todo");

    Ok(())
}
