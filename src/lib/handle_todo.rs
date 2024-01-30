use super::Todo;

use dialoguer::{theme::ColorfulTheme, Select};

pub fn handle_todo(todos: &mut Vec<Todo>, index: usize) {
    let todo = &mut todos[index];

    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action:")
        .items(&[
            if todo.completed {
                "Mark as uncompleted"
            } else {
                "Mark as completed"
            },
            "Remove",
            "Exit",
        ])
        .interact()
        .expect("Failed to select action");

    match action {
        0 => todo.toggle_status(),
        1 => drop(todos.remove(index)),
        _ => (),
    }
}
