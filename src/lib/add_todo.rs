use dialoguer::{theme::ColorfulTheme, Input};
use std::{fs, io::Write, path::Path};

pub fn add_todo() {
    let todo: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Add todo:")
        .interact_text()
        .expect("Failed to read input");

    if !Path::new("todos.txt").exists() {
        fs::write("todos.txt", todo).expect("Failed to write to file");
        return;
    }

    let mut data_file = fs::OpenOptions::new()
        .append(true)
        .open("todos.txt")
        .expect("Failed to open file");

    data_file
        .write(format!("\n{}", todo).as_bytes())
        .expect("Failed to write to file");
}
