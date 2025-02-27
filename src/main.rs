mod structs;
mod data;
mod config;
mod cli_reader;

pub use structs::*;

use std::fs::{self, File};
use std::path::Path;
use std::io::Write;

fn main() {
	let match_result = cli_reader::parse_commands();
	
	if let Some(matches_add) = match_result.subcommand_matches("add") {
		//"$ spaced add" was run
		let mut data: Data = get_data();
		if let Some(title) = matches_add.get_one::<String>("title") {
			//"$ spaced add ... ..." was run
			println!("Adding todo...");
			//if let Some(tags) = matches_add.get_many::<String>("tags").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>() {
			let tags: Option<Vec<String>> = matches_add
                .get_many::<String>("tags")
                .map(|values| values.map(|v| v.to_string()).collect::<Vec<String>>());
			if let Some(tag_vec) = tags {
				//"$ spaced add ... -t ... ..." was run
				data.add_todo(title, Some(tag_vec).unwrap());
			} else {
				data.add_todo(title, <Option<Vec<String>>>::None.unwrap());
				println!("No tags provided, 1 recommended");
			}
		} else {
			println!("Not printing testing lists...");
		}
	}

	if let Some(matches_ls) = match_result.subcommand_matches("ls") {
		//"$ spaced ls ..." was run

		let tags: Option<Vec<String>> = matches_ls
			.get_many::<String>("tags")
			.map(|values| values.map(|v| v.to_string()).collect::<Vec<String>>());

		let which = matches_ls.get_one::<String>("which"); //"$ spaced ls ... ..." was run
		list_data(which, tags);
	}

	if let Some(matches_rm) = match_result.subcommand_matches("rm") {
		//"$ spaced rm ..." was run
		let item_id = *matches_rm.get_one::<u32>("item_id").unwrap();
		remove_item(item_id);
	}

	if let Some(matches_check) = match_result.subcommand_matches("check") {
		//"$ spaced check ..." was run
		let todo_id = *matches_check.get_one::<u32>("todo_id").unwrap();
		set_todo_done(todo_id);
	}
}

fn remove_item(item_id: u32) {
	let mut data: Data = get_data();
	
	if let Err(e) = data.remove_item(item_id) {
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
        //println!("Todo with id {} marked as done and moved to spaced repetition", todo_id);
    }
}

fn list_data(which: Option<&String>, tags: Option<Vec<String>>) {
	let data = get_data();
	if which.is_none() {
		data.list_todos(tags.clone());
		println!();
		data.list_spaced_repetition(tags.clone());
		return;
	}
	match which.unwrap().as_str() {
		"all" | "void" => {
			data.list_todos(tags.clone());
			println!();
			data.list_spaced_repetition(tags.clone());
		}

		"todo" => {
			data.list_todos(tags.clone())
		}
		
		"spaced" | "spaced_repetition" | "repetition" => {
			data.list_spaced_repetition(tags.clone())
		}

		_ => {
			println!("Invalid argument. Please use 'todo', 'spaced' or nothing");
		}
	}
}

fn get_data() -> Data {
    let file_path = "./data/data.yaml";
    let dir_path = Path::new(file_path).parent().unwrap();
    
    // Check if the data file exists
    if !Path::new(file_path).exists() {
        println!("Data file not found at {}", file_path);
        
        // Create directory if it doesn't exist
        if !dir_path.exists() {
            //println!("Creating directory: {}", dir_path.display());
            fs::create_dir_all(dir_path).expect("Failed to create data directory");
        }
        
        // Create an empty data file
        //println!("Creating new data file");
        let default_data = Data::default();
        let yaml_content = serde_yaml::to_string(&default_data).expect("Failed to serialize default data");
        
        let mut file = File::create(file_path).expect("Failed to create data file");
        file.write_all(yaml_content.as_bytes()).expect("Failed to write to data file");
        
        println!("Initialized new data file at {}", file_path);
        
        return default_data;
    }
    
    // File exists, try loading it
    match Data::load(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading data file: {}", e);
            println!("Using default data instead");
            Data::default()
        }
    }
}