use lib::{
    get_todos::get_todos,
    handle_todo::HandleTodo,
    save_to_file::SaveToFile,
    select_action::{select_action, Selection},
    todo::Todo,
};

fn main() {
    let mut todos = get_todos();

    loop {
        todos.sort_by(|a, b| a.completed.cmp(&b.completed));

        match select_action(&todos) {
            Selection::Ignore => (),
            Selection::Add => todos.push(Todo::new()),
            Selection::Exit => {
                todos.save_to_file();
                break;
            }
            Selection::Todo(index) => todos.handle_todo(index),
        }
    }
}
