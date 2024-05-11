use std::fs::read;
use crate::prelude::*;
use super::cli;

const CONFIG_PATH: &str = "./oxydian/config.json";

#[derive(Clone, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct AppConfig {
	pub vault_path: String,
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
		let config = serde_json::from_slice::<AppConfig>(&config_file)?;
		Ok(App { config, flows })
	}

	pub fn register_flow(mut self, name: &str, flow: Flow) -> Self {
		self.flows.insert(name.to_string(), flow).expect("Flow already exists");
		return self;
	}

	pub fn execute (&self) -> Result<()> {
		let args = cli::parse()?;
		let config = self.config.clone();
		let origin = args.origin;
		let vault_path = PathBuf::from(&config.vault_path);
		let vault = Vault { vault_path: vault_path, origin };
		let flow = self.flows.get(&args.flow).ok_or(anyhow!("Flow not found"))?;
		flow.execute(&vault, None) // todo: turn origin into a Note
	}
}
