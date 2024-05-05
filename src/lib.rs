use clap::Parser;


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[arg(long, short)]
	dry_run: bool,
	flow: String,
	origin: Option<String>,
}


fn main() {
	let args = Cli::parse();
	println!("{:?}", args);
}
