use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new() -> Self {
        let todo = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Add todo:")
            .interact_text()
            .expect("Failed to read input");

        return Self {
            title: todo,
            completed: false,
        };
    }

    pub fn toggle_status(&mut self) {
        self.completed = !self.completed;
    }

    pub fn get_formatted_title(&self) -> String {
        if self.completed {
            return format!("{} - ✅", self.title);
        }

        return self.title.to_string();
    }
}
