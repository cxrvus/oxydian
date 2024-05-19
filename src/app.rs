use crate::{flow::Flow, util::*, vault::*};
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
	pub note: Option<PathBuf>,
}

pub fn parse_args() -> Result<Cli> {
	Cli::try_parse().map_err(|e| anyhow!(e))
}


pub type FlowMap = HashMap<String, Flow>;

pub struct App {
	config: Vault,
	flows: FlowMap,
}

impl App {
	pub fn new(vault_setup: VaultConfig) -> Result<Self> {
		Ok(Self {
			config: Vault::from(vault_setup),
			flows: FlowMap::new()
		})
	}

	pub fn with_flows(mut self, flows: FlowMap) -> Result<Self> {
		for (name, flow) in flows {
			self = self.register_flow(&name, flow)?;
		}
		Ok(self)
	}

	pub fn register_flow(mut self, name: &str, flow: Flow) -> Result<Self> {
		let old_entry = self.flows.insert(name.into(), flow);
		if old_entry.is_some() { Err(anyhow!("Flow with name {name} already exists")) }
		else { Ok(self) }
	}

	pub fn execute (&self) -> Result<()> {
		let command = parse_args()?.command;
		match command {
			Command::ExecuteFlow(args) => self.run_flow(args),
		}
	}

	fn run_flow(&self, args: FlowArgs) -> Result<()> {
		let flow = self.flows.get(&args.flow).ok_or(anyhow!("Flow not found"))?;
		let config = &self.config;
		let note = args.note;
		flow.execute(config, note) 
	}
}
