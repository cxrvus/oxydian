use crate::{context::Context, file::File, flow::*, util::*, vault::*};
use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
	// clap subcommand called "flow"
	#[command(subcommand)]
	pub command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
	#[clap(name = "flow", about = "Run a flow")]
	ExecuteFlow(FlowArgs),
}

#[derive(Parser, Debug)]
struct FlowArgs {
	#[arg(required = true)]
	pub flow: String,
	pub file_path: Option<PathBuf>,
}

fn parse_args() -> Result<Cli> {
	Cli::try_parse().map_err(|e| anyhow!(e))
}


pub fn execute (vault: Vault, flows: FlowController) -> Result<()> {
	let command = parse_args()?.command;
	match command {
		Command::ExecuteFlow(FlowArgs { flow, file_path }) => { 
			let file = file_path
				.ok_or(anyhow!("No origin file provided"))
				.map(|file_path| { 
					if file_path.is_absolute() { file_path }
					else { vault.root_path().join(file_path) }
				})
				.map(|file_path| File::get(file_path).map_err(|_| anyhow!("Failed to get origin note file")))
			?;

			let ctx = Context { vault, file };
			flows.execute(&flow, &ctx)
		}
	}
}
