use std::fs::read;
use crate::prelude::*;
use super::cli;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
struct AppConfig {
	vault_dir: String,
}

pub struct App {
	config: AppConfig,
	flows: HashMap<String, Flow>
}

impl App {
	pub fn new() -> Result<Self> {
		let config_file = read("./oxyconfig.json").expect("Please create a oxyconfig.json file in the root directory of your project.");
		let config = serde_json::from_slice::<AppConfig>(&config_file)?;
		let flows = HashMap::new();
		println!("{}", config.vault_dir);
		return Ok(App { config, flows });
	}

	// pub fn execute(&self) {
	// 	let actions = self.get_actions();
	// 	match actions {
	// 		Ok(actions) => { self.invoke_actions(actions) },
	// 		Err(e) => { println!("Error: {}", e.to_string()) },
	// 	}
	// }

	// fn get_actions (&self) -> Result<Vec<FileAction>> {
	// 	let args = cli::parse()?;
	// 	let flow = self.flows.get(&args.flow).ok_or(anyhow!("Flow not found"))?;
	// 	let actions = flow.invoke()?;
	// 	Ok(actions)
	// }

	// fn invoke_actions (&self, actions: Vec<FileAction>) {
	// 	todo!("invoke actions")
	// }

	pub fn register_flow(mut self, name: &str, flow: Flow) -> Self {
		self.flows.insert(name.to_string(), flow).expect("Flow already exists");
		return self;
	}
}
