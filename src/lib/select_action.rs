use super::Todo;

use dialoguer::{theme::ColorfulTheme, Select};

pub enum Selection {
    Ignore,
    Add,
    Exit,
    Todo(usize),
}

pub fn select_action(todos: &Vec<Todo>) -> Selection {
    let mut select_from = Vec::with_capacity(todos.len() + 3);
    select_from.push(String::new());

    select_from.extend(todos.iter().map(|todo| todo.get_formatted_title()));
    select_from.extend(["Add".to_string(), "Exit".to_string()]);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&select_from)
        .default(1)
        .interact()
        .expect("Failed to select todo");

    if selection == 0 {
        return Selection::Ignore;
    }

    if selection == select_from.len() - 1 {
        return Selection::Exit;
    }

    if selection == select_from.len() - 2 {
        return Selection::Add;
    }

    return Selection::Todo(selection - 1);
}
