use super::todo::Todo;

pub fn get_todos() -> Vec<Todo> {
    let json = std::fs::read_to_string("todos.json").unwrap_or("[]".to_string());
    let todos = serde_json::from_str(&json).expect("Failed to parse json");

    return todos;
}
