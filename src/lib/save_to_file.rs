use super::todo::Todo;
use serde_json;

pub trait SaveToFile {
    fn save_to_file(&self);
}

impl SaveToFile for Vec<Todo> {
    fn save_to_file(&self) {
        let json_string = serde_json::to_string(&self).expect("Failed to serialize todos");
        std::fs::write("todos.json", json_string).expect("Failed to write to file");
    }
}
