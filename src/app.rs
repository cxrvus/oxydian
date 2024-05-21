use crate::{flow::*, util::*, vault::*};
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
	vault: Vault,
	flows: FlowController,
}

impl App {
	pub fn new(vault: Vault) -> Result<Self> {
		Ok(Self {
			vault,
			flows: Default::default(),
		})
	}

	pub fn execute (&self) -> Result<()> {
		let command = parse_args()?.command;
		match command {
			Command::ExecuteFlow(FlowArgs { flow, file_path }) => self.flows.run(flow, &self.vault, file_path),
		}
	}

	pub fn register_flows(mut self, flows: Vec<Flow>) -> Result<Self> {
		self.flows = self.flows.register_flows(flows)?;
		Ok(self)
	}

	pub fn register_flow(mut self, flow: Flow) -> Result<Self> {
		self.flows = self.flows.register_flow(flow)?;
		Ok(self)
	}
}
