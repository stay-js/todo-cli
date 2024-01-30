use super::Todo;

use dialoguer::{theme::ColorfulTheme, Input};

pub fn add_todo(todos: &mut Vec<Todo>) {
    let todo = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Add todo:")
        .interact_text()
        .expect("Failed to read input");

    todos.push(Todo {
        title: todo,
        completed: false,
    });
}
