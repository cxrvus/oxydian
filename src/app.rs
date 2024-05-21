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
	#[arg(long, short)]
	pub dry_run: bool,
	#[arg(required = true)]
	pub flow: String,
	pub note_path: Option<PathBuf>,
}

pub fn parse_args() -> Result<Cli> {
	Cli::try_parse().map_err(|e| anyhow!(e))
}


pub struct App {
	vault: Vault,
	flows: FlowMap,
}

impl App {
	pub fn new(vault_setup: VaultConfig) -> Result<Self> {
		Ok(Self {
			vault: Vault::from(vault_setup),
			flows: FlowMap::new()
		})
	}

	pub fn with_flows(mut self, flows: FlowList) -> Result<Self> {
		for flow in flows {
			self = self.register_flow(flow)?;
		}
		Ok(self)
	}

	pub fn register_flow(mut self, flow: Flow) -> Result<Self> {
		let name = flow.name;
		if self.flows.contains_key(name) { return Err(anyhow!("Flow with name {name} already exists")); }
		else { self.flows.insert(name.to_string(), flow); }
		Ok(self)
	}

	pub fn execute (&self) -> Result<()> {
		let command = parse_args()?.command;
		match command {
			Command::ExecuteFlow(args) => self.run_flow(args),
		}
	}

	fn run_flow(&self, args: FlowArgs) -> Result<()> {
		let flow = self.flows.get(&args.flow).ok_or(anyhow!("Flow not found"))?;
		let vault = &self.vault;
		let note_path = args.note_path.map(|path| { 
			if path.is_absolute() { path }
			else { vault.root_path().join(path) }
		});
		flow.func.execute(vault, note_path) 
	}
}
