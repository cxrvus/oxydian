use crate::{cli::*, flow::IFlow, util::*, vault_config::*};


pub struct Vault {
	config: VaultConfig,
	flows: HashMap<String, IFlow>
}


impl Vault {
	pub fn new(vault_setup: VaultSetup) -> Result<Self> {
		Ok(Self {
			config: VaultConfig::from(vault_setup),
			flows: HashMap::new()
		})
	}

	pub fn with_flows(mut self, flows: HashMap<String, IFlow>) -> Self {
		for (name, flow) in flows {
			self.flows.insert(name, flow);
		}
		return self;
	}

	pub fn register_flow(mut self, name: &str, flow: IFlow) -> Self {
		self.flows.insert(name.to_string(), flow).expect(&format!("Flow {name} already exists"));
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
