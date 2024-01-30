use lib::{
    add_todo::add_todo,
    get_todos::get_todos,
    handle_todo::handle_todo,
    select_action::{select_action, Selection},
    Save,
};

fn main() {
    let mut todos = get_todos();

    loop {
        if todos.len() == 0 {
            println!("\nYou have no todos! Done for now :)\n");
        }

        todos.sort_by(|a, b| a.completed.cmp(&b.completed));

        let selection = select_action(&todos);

        match selection {
            Selection::Exit => {
                todos.save();
                break;
            }
            Selection::Add => add_todo(&mut todos),
            Selection::Todo(index) => handle_todo(&mut todos, index),
        }
    }
}
