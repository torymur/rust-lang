//! This crate provides an API to parse list of todos

use std::fs::read_to_string;
use std::path::Path;

mod error;
use error::ParseErr;
use error::ReadErr;

use std::error::Error;

/// This struc contains a list of todos parsed as a Vec<String>
#[derive(Debug)]
pub struct TodoList {
    tasks: Vec<String>,
}

impl TodoList {
    pub fn get_todos<P>(path: P) -> Result<TodoList, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let read_todos: Result<String, Box<dyn Error>> = Self::read_todos(path);
        let parsed_todos = Self::parse_todos(&read_todos?)?;
        Ok(parsed_todos)
    }

    pub fn read_todos<P>(path: P) -> Result<String, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let raw_todos = read_to_string(path).map_err(|e| ReadErr {
            child_err: Box::new(e),
        })?;
        Ok(raw_todos)
    }

    pub fn parse_todos(todo_str: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut tasks: Vec<String> = vec![];
        for i in todo_str.lines() {
            tasks.push(i.to_string());
        }
        if tasks.is_empty() {
            Err(ParseErr::Empty.into())
        } else {
            Ok(TodoList { tasks })
        }
    }
}
