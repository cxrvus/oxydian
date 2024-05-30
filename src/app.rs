use crate::{context::Context, file::File, flow::*, util::*, vault::*};
use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
	// clap subcommand called "flow"
	#[command(subcommand)]
	pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
	#[clap(name = "flow", about = "Run a flow")]
	ExecuteFlow(FlowArgs),
}

#[derive(Parser, Debug)]
pub struct FlowArgs {
	#[arg(required = true)]
	pub flow: String,
	pub file_path: Option<PathBuf>,
}

pub fn parse_args() -> Result<Cli> {
	Cli::try_parse().map_err(|e| anyhow!(e))
}


pub struct App {
	pub vault: Vault,
	pub flows: FlowController,
}

impl App {
	pub fn execute (&self) -> Result<()> {
		let command = parse_args()?.command;
		match command {
			Command::ExecuteFlow(FlowArgs { flow, file_path }) => { 
				let file = file_path
					.ok_or(anyhow!("No origin file provided"))
					.map(|file_path| { 
						if file_path.is_absolute() { file_path }
						else { self.vault.root_path().join(file_path) }
					})
					.map(|file_path| File::get(file_path).map_err(|_| anyhow!("Failed to get origin note file")))
					// .map(|file| &mut file)
				?;


				let vault = self.vault;
				// todo: turn file into a normal owned field & try passing ctx as mutable
				let ctx = Context { vault, file };
				self.flows.execute(&flow, &ctx)
			}
		}
	}
}
