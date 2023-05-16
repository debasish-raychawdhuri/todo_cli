//create a REPL for todo management

use rpassword::read_password;
use std::error::Error;
#[derive(Debug)]
enum ReplCommand<'a> {
    CreateUser(&'a str, &'a str),
    ChangePassword(&'a str, String, String),
    CreateTodo(&'a str),
    SearchTodo(&'a str),
    EditTodo(i32, &'a str),
    DeleteTodo(i32),
    MarkTodoAsDone(i32),
    ListAllTodos,
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

fn read_command_line() -> String {
    print!("> ");
    let mut command_line = String::new();
    std::io::stdin()
        .read_line(&mut command_line)
        .expect("Failed to read line");
    command_line.trim().to_string()
}

pub fn repl_loop() {
    loop {
        let command_line = read_command_line();
        let command = read_parse_repl_command(&command_line);
        match command {
            Ok(ReplCommand::CreateUser(username, password)) => {
                println!("Creating user {} with password {}", username, password);
            }
            Ok(ReplCommand::ChangePassword(username, old_password, new_password)) => {
                println!(
                    "Changing password for user {} from {} to {}",
                    username, old_password, new_password
                );
            }
            Ok(ReplCommand::CreateTodo(description)) => {
                println!("Creating todo with description {}", description);
            }
            Ok(ReplCommand::SearchTodo(description)) => {
                println!("Searching todo with description {}", description);
            }
            Ok(ReplCommand::EditTodo(id, description)) => {
                println!("Editing todo {} with description {}", id, description);
            }
            Ok(ReplCommand::DeleteTodo(id)) => {
                println!("Deleting todo {}", id);
            }
            Ok(ReplCommand::MarkTodoAsDone(id)) => {
                println!("Marking todo {} as done", id);
            }
            Ok(ReplCommand::ListAllTodos) => {
                println!("Listing all todos");
            }
            Ok(ReplCommand::Exit) => {
                println!("Exiting");
                return;
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

    println!("command: {}", command);
    println!("args: {:?}", args);

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
            args[0],
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
    } else if command == "exit" {
        if args.len() != 0 {
            return Err("exit command takes 0 argument".into());
        }
        Ok(ReplCommand::Exit)
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
