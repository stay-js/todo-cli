use dialoguer::{theme::ColorfulTheme, Select};

pub enum Action {
    Add,
    Remove,
    Exit,
}

pub fn select_action(number_of_todos: usize) -> Action {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action:")
        .items(if number_of_todos == 0 {
            &["Add", "Exit"]
        } else {
            &["Add", "Remove", "Exit"]
        })
        .interact()
        .expect("Failed to select action");

    if number_of_todos == 0 {
        return match selection {
            0 => Action::Add,
            1 => Action::Exit,
            _ => panic!("Invalid selection"),
        };
    }

    return match selection {
        0 => Action::Add,
        1 => Action::Remove,
        2 => Action::Exit,
        _ => panic!("Invalid selection"),
    };
}
