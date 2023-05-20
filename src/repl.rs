//create a REPL for todo management

use crate::control::{
    change_user_password, create_new_todo, create_new_user, delete_todo,
    get_all_pending_todos_for_user, get_all_todos_for_user, get_username_for_user_id,
    mark_todo_done, search_todo_by_description, update_todo,
};
use diesel::PgConnection;
use rpassword::read_password;
use std::{error::Error, io::Write};

#[derive(Debug)]
enum ReplCommand<'a> {
    CreateUser(&'a str, &'a str),
    ChangePassword(String, String),
    CreateTodo(&'a str),
    SearchTodo(&'a str),
    EditTodo(i32, &'a str),
    DeleteTodo(i32),
    MarkTodoAsDone(i32),
    ListAllTodos,
    ListAllPendingTodos,
    Help,
    Exit,
}

struct TokenIterator<'a> {
    sentence: &'a str,
    index: usize,
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        enum State {
            InWord,
            InWhitespace,
            InSingleQuote,
            InDoubleQuote,
        }

        let mut state = State::InWhitespace;
        let mut start = self.index;

        loop {
            if self.index >= self.sentence.len() {
                if start == self.index {
                    return None;
                } else {
                    return Some(&self.sentence[start..self.index]);
                }
            }
            match &state {
                State::InWord => {
                    let c = self.sentence.chars().nth(self.index)?;
                    if c.is_whitespace() {
                        return Some(&self.sentence[start..self.index]);
                    } else {
                        self.index += 1;
                    }
                }
                State::InWhitespace => {
                    let c = self.sentence.chars().nth(self.index)?;
                    if c.is_whitespace() {
                        self.index += 1;
                    } else if c == '\'' {
                        state = State::InSingleQuote;
                        self.index += 1;
                        start = self.index;
                    } else if c == '"' {
                        state = State::InDoubleQuote;
                        self.index += 1;
                        start = self.index;
                    } else {
                        state = State::InWord;
                        start = self.index;
                    }
                }
                State::InSingleQuote => {
                    let c = self.sentence.chars().nth(self.index)?;
                    if c == '\'' {
                        self.index += 1;
                        return Some(&self.sentence[start..self.index - 1]);
                    } else {
                        self.index += 1;
                    }
                }
                State::InDoubleQuote => {
                    let c = self.sentence.chars().nth(self.index)?;
                    if c == '"' {
                        self.index += 1;
                        return Some(&self.sentence[start..self.index - 1]);
                    } else {
                        self.index += 1;
                    }
                }
            }
        }
    }
}

pub fn login(connection: &mut PgConnection) -> Result<(), Box<dyn Error>> {
    let user = read_username();
    let password = read_pass()?;
    let user_id = authenticate_user(connection, &user, &password)?;
    repl_loop(user_id, connection);
    Ok(())
}
fn read_username() -> String {
    print!("Enter username: ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut username = String::new();
    std::io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    username.trim().to_string()
}
fn read_pass() -> Result<String, Box<dyn Error>> {
    print!("Enter password: ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let password = read_password()?;
    Ok(password)
}

fn authenticate_user(
    conn: &mut PgConnection,
    username: &str,
    password: &str,
) -> Result<i32, Box<dyn Error>> {
    crate::control::authenticate_user(conn, username, password)
}

fn read_command_line(username: &str) -> String {
    print!("{}> ", username);
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut command_line = String::new();
    std::io::stdin()
        .read_line(&mut command_line)
        .expect("Failed to read line");
    command_line.trim().to_string()
}

pub fn repl_loop(user_id: i32, conn: &mut PgConnection) {
    let username = get_username_for_user_id(conn, user_id).expect("Failed to get username");
    loop {
        let command_line = read_command_line(&username);
        let command = read_parse_repl_command(&command_line);
        match command {
            Ok(ReplCommand::CreateUser(username, password)) => {
                match create_new_user(conn, user_id, username, password) {
                    Ok(user) => println!("User with id {} created successfully", user.id),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Ok(ReplCommand::ChangePassword(old_password, new_password)) => {
                match change_user_password(conn, user_id, &old_password, &new_password) {
                    Ok(()) => println!("Password changed successfully"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Ok(ReplCommand::CreateTodo(description)) => {
                match create_new_todo(conn, user_id, description) {
                    Ok(todo) => println!("Todo with id {} created successfully", todo.id),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Ok(ReplCommand::SearchTodo(description)) => {
                match search_todo_by_description(conn, user_id, description) {
                    Ok(todos) => {
                        println!("Found {} todos", todos.len());
                        for todo in todos {
                            println!("{}", todo);
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            Ok(ReplCommand::EditTodo(id, description)) => {
                match update_todo(conn, user_id, id, description) {
                    Ok(()) => println!("Todo edited successfully"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Ok(ReplCommand::DeleteTodo(id)) => match delete_todo(conn, user_id, id) {
                Ok(()) => println!("Todo deleted successfully"),
                Err(e) => println!("Error: {}", e),
            },
            Ok(ReplCommand::MarkTodoAsDone(id)) => match mark_todo_done(conn, user_id, id) {
                Ok(()) => println!("Todo marked as done successfully"),
                Err(e) => println!("Error: {}", e),
            },
            Ok(ReplCommand::ListAllTodos) => match get_all_todos_for_user(conn, user_id) {
                Ok(todos) => {
                    println!("Found {} todos", todos.len());
                    for todo in todos {
                        println!("{} ", todo);
                    }
                }
                Err(e) => println!("Error: {}", e),
            },
            Ok(ReplCommand::ListAllPendingTodos) => {
                match get_all_pending_todos_for_user(conn, user_id) {
                    Ok(todos) => {
                        for todo in todos {
                            println!("{} ", todo);
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            Ok(ReplCommand::Exit) => {
                println!("Exiting");
                return;
            }
            Ok(ReplCommand::Help) => {
                println!("Commands:");
                println!("create-user <username> <password>");
                println!("cu <username> <password>");
                println!("change-password <username> <old-password> <new-password>");
                println!("cp <username> <old-password> <new-password>");
                println!("create-todo <description>");
                println!("ct <description>");
                println!("search-todo <description>");
                println!("st <description>");
                println!("edit-todo <id> <description>");
                println!("et <id> <description>");
                println!("delete-todo <id>");
                println!("dt <id>");
                println!("mark-todo-as-done <id>");
                println!("md <id>");
                println!("list-all-todos");
                println!("lt");
                println!("list-all-pending-todos");
                println!("lp");
                println!("help");
                println!("exit");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn read_parse_repl_command<'a>(command: &'a str) -> Result<ReplCommand<'a>, Box<dyn Error>> {
    let mut iter = TokenIterator {
        sentence: command,
        index: 0,
    };
    let command = iter.next().ok_or("No command found")?;
    let args = iter.collect::<Vec<&str>>();

    if command == "create-user" || command == "cu" {
        if args.len() != 2 {
            return Err("create-user command takes 2 arguments".into());
        }
        Ok(ReplCommand::CreateUser(args[0], args[1]))
    } else if command == "change-password" || command == "cp" {
        if args.len() != 1 {
            return Err("change-password command takes 3 arguments".into());
        }
        print!("Enter old password: ");
        let old_password = read_password()?;
        print!("Enter new password: ");
        let new_password = read_password()?;
        Ok(ReplCommand::ChangePassword(
            old_password.to_string(),
            new_password.to_string(),
        ))
    } else if command == "create-todo" || command == "ct" {
        if args.len() != 1 {
            return Err("create-todo command takes 1 argument".into());
        }
        Ok(ReplCommand::CreateTodo(args[0]))
    } else if command == "search-todo" || command == "st" {
        if args.len() != 1 {
            return Err("search-todo command takes 1 argument".into());
        }
        Ok(ReplCommand::SearchTodo(args[0]))
    } else if command == "edit-todo" || command == "et" {
        if args.len() != 2 {
            return Err("edit-todo command takes 2 arguments".into());
        }
        let id = args[0].parse::<i32>()?;
        Ok(ReplCommand::EditTodo(id, args[1]))
    } else if command == "delete-todo" || command == "dt" {
        if args.len() != 1 {
            return Err("delete-todo command takes 1 argument".into());
        }
        let id = args[0].parse::<i32>()?;
        Ok(ReplCommand::DeleteTodo(id))
    } else if command == "mark-todo-as-done" || command == "md" {
        if args.len() != 1 {
            return Err("mark-todo-as-done command takes 1 argument".into());
        }
        let id = args[0].parse::<i32>()?;
        Ok(ReplCommand::MarkTodoAsDone(id))
    } else if command == "list-all-todos" || command == "lt" {
        if args.len() != 0 {
            return Err("list-all-todos command takes 0 argument".into());
        }
        Ok(ReplCommand::ListAllTodos)
    } else if command == "list-all-pending-todos" || command == "lp" {
        if args.len() != 0 {
            return Err("list-all-todos command takes 0 argument".into());
        }
        Ok(ReplCommand::ListAllPendingTodos)
    } else if command == "exit" {
        if args.len() != 0 {
            return Err("exit command takes 0 argument".into());
        }
        Ok(ReplCommand::Exit)
    } else if command == "help" {
        if args.len() != 0 {
            return Err("help command takes 0 argument".into());
        }
        Ok(ReplCommand::Help)
    } else {
        Err("Unknown command".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_iterator() {
        let mut iter = TokenIterator {
            sentence: "Hello, 'world'",
            index: 0,
        };
        assert_eq!(iter.next(), Some("Hello,"));
        assert_eq!(iter.next(), Some("world"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_token_iterator2() {
        let mut iter = TokenIterator {
            sentence: "Hello, \"world\"",
            index: 0,
        };
        assert_eq!(iter.next(), Some("Hello,"));
        assert_eq!(iter.next(), Some("world"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_token_iterator3() {
        let mut iter = TokenIterator {
            sentence: "Hello, \"world is man's world!\"",
            index: 0,
        };
        assert_eq!(iter.next(), Some("Hello,"));
        assert_eq!(iter.next(), Some("world is man's world!"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_token_iterator4() {
        let mut iter = TokenIterator {
            sentence: "Hello, Tom's mom!",
            index: 0,
        };
        assert_eq!(iter.next(), Some("Hello,"));
        assert_eq!(iter.next(), Some("Tom's"));
        assert_eq!(iter.next(), Some("mom!"));
        assert_eq!(iter.next(), None);
    }
}
