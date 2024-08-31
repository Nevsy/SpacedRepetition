use clap::{Parser, Subcommand};

//use crate::config::Config;
mod structs;
mod data;
mod config;

pub use structs::*;

#[derive(Parser)]
#[command(name = "Repeat", bin_name = "spacedRepetition", author, version)]
#[command(about = "Spaced repetition CLI or TUI idk yet, using Rust \n Allows you to manage your todos, but mostly your spaced repetition :)", long_about = None)]

struct Cli {
	/// Turn debugging information on
	#[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

	#[arg(short, long)]
	tags: Vec<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// Adds a new todo
	Add {
		title: String
	},

	/// Marks a todo as done
	Check {
		todo_id: u32
	},

	/// Lists all todos
	Ls {
		which: Option<String>
	},

	Rm {
		item_id: u32
	}
}

fn main() {
	let cli = Cli::parse();

	match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

	match &cli.command {
		Some(Commands::Add { title}) => {
			add_todo(title, &cli);	
		}

		Some(Commands::Check { todo_id }) => {
			set_todo_done(*todo_id);
		}

		Some (Commands::Ls { which }) => {
			list_data(which);
		},

		Some(Commands::Rm { item_id }) => {
			remove_item(item_id);
		},

        None => {
			let data = get_data();
			data.list_todos();
			data.check_to_revise();
			println!("\nNo command specified");
		}
    }
}

fn add_todo(title: &str, cli: &Cli) {
    let mut data = get_data();
	let mut next_id = 0;

	for (_i, todo) in data.todos.iter().enumerate() {
		if todo.id >= next_id {
			next_id = todo.id + 1;
		}
	}
	for (_i, item) in data.spaced_repetition.iter().enumerate() {
		if item.id >= next_id {
			next_id = item.id + 1;
		}
	}

    let new_todo = Todo {
        id: next_id,
        title: title.to_string(),
        created: chrono::Local::now(),
        updated: chrono::Local::now(),
        priority: Some(0),
        tags: cli.tags.clone(),
    };

	for tag in &new_todo.tags {
		if !data.tags_ease.iter().any(|t: &TagsEase| t.tag == *tag) {
			println!("Tag '{}' not found in config, adding it", tag);
			data.add_tag(tag.to_string());
			continue;
		}
	}

    data.todos.push(new_todo);
    data.save("./data/data.yaml").expect("Failed to save todo");
    println!("Todo added: {} -> {}", title, next_id);
}

fn remove_item(item_id: &u32) {
	let mut data: Data = get_data();
	
	if let Err(e) = data.remove_item(*item_id) {
		eprintln!("{}", e);
	} else {
		data.save("./data/data.yaml").expect("Failed to save data");
		println!("Todo with id {} removed", item_id);
	}
}

fn set_todo_done(todo_id: u32) {
	let mut data: Data = get_data();
    
    if let Err(e) = data.move_to_spaced_repetition(todo_id) {
        eprintln!("{}", e);
    } else {
        data.save("./data/data.yaml").expect("Failed to save data");
        println!("Todo with id {} marked as done and moved to spaced repetition", todo_id);
    }
}

fn list_data(which: &Option<String>) {
	let data = get_data();
	match which.as_deref() {
		None | Some("all") | Some("void") => {
			data.list_todos();
			println!();
			data.list_spaced_repetition();
		}

		Some("todo") => {
			data.list_todos()
		}
		
		Some("spaced") => {
			data.list_spaced_repetition()
		}

		_ => {
			// TODO: listing by tag
			println!("Invalid argument. Please use 'todo', 'spaced' or nothing");
		}
	}
}

fn get_data() -> Data {
	return Data::load("./data/data.yaml").unwrap_or_default();
}
