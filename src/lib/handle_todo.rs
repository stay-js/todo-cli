use super::Todo;

use dialoguer::{theme::ColorfulTheme, Select};

pub fn handle_todo(todos: &mut Vec<Todo>, index: usize) {
    let todo = &mut todos[index];

    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action:")
        .items(&[
            if todo.completed {
                "Mark as uncomplete"
            } else {
                "Mark as completed"
            },
            "Remove",
            "Exit",
        ])
        .interact()
        .expect("Failed to select action");

    if action == 0 {
        todo.toggle_status();
    } else if action == 1 {
        todos.remove(index);
    }
}
