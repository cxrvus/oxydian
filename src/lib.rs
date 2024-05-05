use clap::Parser;


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[arg(long, short)]
	dry_run: bool,
	flow: String,
	origin: Option<String>,
}


pub fn execute() {
	let args = Cli::parse();
	println!("{:?}", args);
}
