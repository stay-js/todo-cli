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
}

pub trait Save {
    fn save(&self);
}

impl Save for Vec<Todo> {
    fn save(&self) {
        std::fs::write("todos.json", serde_json::to_string(&self).unwrap())
            .expect("Failed to write to file");
    }
}
