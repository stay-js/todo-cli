use super::todo::Todo;
use dialoguer::{theme::ColorfulTheme, Select};

pub trait HandleTodo {
    fn handle_todo(&mut self, index: usize);
}

impl HandleTodo for Vec<Todo> {
    fn handle_todo(&mut self, index: usize) {
        let todo = &mut self[index];

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
            1 => drop(self.remove(index)),
            _ => (),
        }
    }
}
