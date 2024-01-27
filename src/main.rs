use lib::{
    add_todo::add_todo,
    remove_todo::remove_todo,
    select_action::{select_action, Action},
};

fn main() {
    loop {
        let todos_as_string = std::fs::read_to_string("todos.txt").unwrap_or(String::new());
        let mut todos: Vec<_> = todos_as_string.trim().lines().collect();

        if todos.len() == 0 {
            println!("\nYou have no todos! Done for now :)\n");
        } else {
            println!("\nTodos:\n- {}\n", todos.join("\n- "));
        }

        match select_action(todos.len()) {
            Action::Add => add_todo(),
            Action::Remove => remove_todo(&mut todos),
            Action::Exit => break,
        }
    }
}
