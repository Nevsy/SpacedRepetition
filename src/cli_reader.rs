use clap::{command, Arg, ArgMatches, Command};

pub fn parse_commands() -> ArgMatches {
	// https://docs.rs/clap/latest/clap/_tutorial/index.html
	let match_result = command!()
		.about("A simple todo list")
		.subcommand(
			Command::new("add")
				.about("Adds a new todo")
				.arg(Arg::new("title")
					.help("The title of the todo")
					.required(true))
				.arg(Arg::new("tags")
					.short('t')
					.long("tags")
					.help("tags to be added to the todo")
					.num_args(1..=64)) // ENABLE MULTIPLE, PERHAPS SPEAK UP WHEN NO TAGS ARE PROVIDED
		)
		.subcommand(
			Command::new("check")
				.about("Marks a *todo* as done")
				.arg(Arg::new("todo_id")
					.help("The id of the todo")
					.required(true)
					.value_parser(clap::value_parser!(u32)))
				.aliases(["tick", "done"])
		)
		.subcommand(
			Command::new("ls")
				.about("Lists all items")
				.arg(Arg::new("which")
					.help("Select which items to list: 'all', 'todos', 'spaced' or leave empty")
					.required(false))
				.arg(Arg::new("tags")
					.help("Select a tag of which to list items")
					.required(false)
					.short('t')
					.long("tags")
					.num_args(1..=64)
					.alias("tag"))
				.aliases(["list", "show"])
		)
		.subcommand(
			Command::new("rm")
				.about("Remove an item")
				.aliases(["remove", "delete"])
				.arg(Arg::new("item_id")
					.help("The id of the item")
					.required(true)
					.value_parser(clap::value_parser!(u32)))
		)
		.subcommand(
			Command::new("revised")
				.about("Sets an *item* as being revised")
				.arg(Arg::new("item_id")
					.help("The id of the item")
					.required(true))
				.arg(Arg::new("ease")
					.help("The ease of the item, an integer from 1 to 3, 3 being the hardest")
					.required(true))
				.aliases(["revise", "rev"])
		)
		.get_matches();

	match_result
}