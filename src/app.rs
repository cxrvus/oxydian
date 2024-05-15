use std::fs::read;
use crate::{cli::{parse_args, Command, FlowArgs}, prelude::*};


const CONFIG_PATH: &str = "./oxydian/config.json";

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct AppConfigRaw {
	pub vault_path: String,
}

#[derive(Clone)]
pub struct AppConfig {
	pub vault_path: PathBuf,
}

impl From<AppConfigRaw> for AppConfig {
	fn from(raw: AppConfigRaw) -> Self {
		AppConfig { vault_path: PathBuf::from(raw.vault_path) }
	}
}

pub struct App {
	config: AppConfig,
	flows: HashMap<String, Flow>
}


impl App {
	pub fn new() -> Result<Self> {
		App::with_flows(HashMap::new())
	}

	pub fn with_flows(flows: HashMap<String, Flow>) -> Result<Self> {
		let config_file = read(CONFIG_PATH).expect("Please create a oxyconfig.json file in the root directory of your project.");
		let config = serde_json::from_slice::<AppConfigRaw>(&config_file)?;
		let config = AppConfig::from(config);
		Ok(App { config, flows })
	}

	pub fn register_flow(mut self, name: &str, flow: Flow) -> Self {
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
		let config = self.config.clone();
		let origin = args.origin;
		let ctx = Context { config, origin };
		let flow = self.flows.get(&args.flow).ok_or(anyhow!("Flow not found"))?;
		flow.execute(&ctx, None) // todo: turn origin into a Note
	}
}
