use clap::{Parser, ValueEnum};


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[arg(long)]
	json: bool,
	#[arg(value_enum)]
	command: Command,
	id: String,
	origin: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Command {
	Flow,
	Query,
}

fn main() {
	let args = Cli::parse();
	println!("JSON: {}\nCommand: {:?}\nID: {}\nOrigin: {}", args.json, args.command, args.id, args.origin.unwrap_or("None".to_string()));
}
