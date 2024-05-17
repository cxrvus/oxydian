use crate::{flow::Flow, util::*, vault_config::*};
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
	Cli::try_parse().context("Failed to parse CLI arguments")
}


pub struct Vault {
	config: VaultConfig,
	flows: HashMap<String, Flow>
}

impl Vault {
	pub fn new(vault_setup: VaultSetup) -> Result<Self> {
		Ok(Self {
			config: VaultConfig::from(vault_setup),
			flows: HashMap::new()
		})
	}

	pub fn with_flows(mut self, flows: HashMap<String, Flow>) -> Self {
		for (name, flow) in flows {
			self.flows.insert(name, flow);
		}
		self
	}

	pub fn register_flow(mut self, name: &str, flow: Flow) -> Self {
		self.flows.insert(name.to_string(), flow).unwrap_or_else(|| panic!("Flow {name} already exists"));
		self
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
