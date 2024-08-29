/*fn main() {
    let cli = Cli::new();
    let config = Config::load("./data/settings.yaml").expect("Failed to load config");
    let mut data = Data::load("./data/data.yaml").expect("Failed to load data");

    let mut spaced_repetition = SpacedRepetition::new(config, data);
    spaced_repetition.run(cli);

    data.save("data.yaml").expect("Failed to save data");
}*/

use clap::{App, Arg, Subcommand};
use chrono::Local;

mod structs;
mod data;

pub use structs::*;
pub use data::*;

fn main() {
    let matches = App::new("Todo CLI")
        .version("1.0")
        .author("Your Name")
        .about("Manages your todos and spaced repetition")
        .subcommand(
            Subcommand::with_name("add")
                .about("Adds a new todo")
                .arg(Arg::with_name("title")
                    .help("Title of the todo")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("due_date")
                    .help("Due date for the todo")
                    .required(true)
                    .index(2)),
        )
        .subcommand(
            Subcommand::with_name("done")
                .about("Marks a todo as done and moves it to spaced repetition")
                .arg(Arg::with_name("todo_id")
                    .help("ID of the todo to mark as done")
                    .required(true)
                    .index(1)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let title = matches.value_of("title").unwrap();
        let due_date = matches.value_of("due_date").unwrap();
        add_todo(title, due_date);
    } else if let Some(matches) = matches.subcommand_matches("done") {
        let todo_id: u32 = matches.value_of("todo_id").unwrap().parse().expect("Invalid ID");
        set_todo_done(todo_id);
    } else {
        eprintln!("No valid subcommand provided");
    }
}

fn add_todo(title: &str, due_date: &str) {
    let mut data = Data::load("data.yaml").unwrap_or_default();
    
    let new_todo = Todo {
        id: data.todos.len() as u32 + 1,
        title: title.to_string(),
        completed: false,
        created: chrono::Local::now().to_string(),
        updated: chrono::Local::now().to_string(),
        due: due_date.to_string(),
        priority: 0,
        tags: Vec::new(),
    };

    data.todos.push(new_todo);
    data.save("data.yaml").expect("Failed to save todo");
    println!("Todo added: {}", title);
}

fn set_todo_done(todo_id: u32) {
    let mut data = Data::load("data.yaml").unwrap_or_default();
    
    if let Err(e) = data.move_to_spaced_repetition(todo_id) {
        eprintln!("{}", e);
    } else {
        data.save("data.yaml").expect("Failed to save data");
        println!("Todo with id {} marked as done and moved to spaced repetition", todo_id);
    }
}
