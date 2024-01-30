pub mod add_todo;
pub mod get_todos;
pub mod handle_todo;
pub mod select_action;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}

impl Todo {
    fn toggle_status(&mut self) {
        self.completed = !self.completed;
    }

    fn get_formatted_title(&self) -> String {
        if self.completed {
            return format!("{} - ✅", self.title);
        }

        return self.title.to_string();
    }
}

pub trait SaveToFile {
    fn save_to_file(&self);
}

impl SaveToFile for Vec<Todo> {
    fn save_to_file(&self) {
        let json_string = serde_json::to_string(&self).expect("Failed to serialize todos");
        std::fs::write("todos.json", json_string).expect("Failed to write to file");
    }
}
