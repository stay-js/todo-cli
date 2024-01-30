use super::Todo;

use dialoguer::{theme::ColorfulTheme, Select};

pub enum Selection {
    Add,
    Exit,
    Todo(usize),
}

pub fn select_action(todos: &Vec<Todo>) -> Selection {
    let mut select_from = todos
        .iter()
        .map(|todo| todo.get_formatted_title())
        .collect::<Vec<_>>();

    select_from.extend(["Add", "Exit"]);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&select_from)
        .interact()
        .expect("Failed to select todo");

    if selection == select_from.len() - 1 {
        return Selection::Exit;
    }

    if selection == select_from.len() - 2 {
        return Selection::Add;
    }

    return Selection::Todo(selection);
}
