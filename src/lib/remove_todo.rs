use dialoguer::{theme::ColorfulTheme, Select};

pub fn remove_todo(todos: &mut Vec<&str>) {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select todo to delete:")
        .items(&todos)
        .interact()
        .expect("Failed to select todo");

    todos.remove(selection);

    std::fs::write("todos.txt", todos.join("\n")).expect("Failed to write to file");
}
