use crate::{cli::{parse_args, Command, FlowArgs}, flow::IFlow, util::*, vault_config::VaultConfig};



pub struct Vault {
	config: VaultConfig,
	flows: HashMap<String, IFlow>
}


impl Vault {
	pub fn new() -> Result<Self> {
		Vault::with_flows(HashMap::new())
	}

	pub fn with_flows(flows: HashMap<String, IFlow>) -> Result<Self> {
		let config = VaultConfig::load()?;
		Ok(Vault { config, flows })
	}

	pub fn register_flow(mut self, name: &str, flow: IFlow) -> Self {
		self.flows.insert(name.to_string(), flow).expect("Flow already exists");
		return self;
	}

	pub fn execute (&self) -> Result<()> {
		let command = parse_args()?.command;
		match command {
			Command::Flow(args) => self.run_flow(args),
		}
	}

	fn run_flow(&self, args: FlowArgs) -> Result<()> {
		let flow = self.flows.get(&args.flow).ok_or(anyhow!("Flow not found"))?;
		let config = &self.config;
		let origin = args.origin;
		flow.execute(&config, origin) 
	}
}
