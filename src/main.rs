use clap::{Parser, Subcommand};

//use crate::config::Config;
mod structs;
mod data;
mod config;

pub use structs::*;

#[derive(Parser)]
#[command(name = "Repeat", bin_name = "spacedRepetition", author, version)]
#[command(about = "Spaced repetition CLI or TUI idk yet, using Rust \n Allows you to manage your todos, but mostly your spaced repetition :)", long_about = None)]

pub struct Cli {
	/// Turn debugging information on
	#[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

	/// ADD: tags to be added to the todo
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

	/// Lists all items
	Ls {
		which: Option<String>
	},

	/// Remove an item
	Rm {
		item_id: u32
	},

	/// Sets an item as being revised
	Revised {
		item_id: u32,
		ease: u8
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
			let mut data: Data = get_data();
			data.add_todo(title, &cli);	
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

		Some(Commands::Revised { item_id, ease }) => {
			let mut data: Data = get_data();
			data.revised_item(*item_id, *ease);
		}

        None => {
			let data = get_data();
			data.list_todos();
			data.check_to_revise();
			println!("\nNo command specified");
		}
    }
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
