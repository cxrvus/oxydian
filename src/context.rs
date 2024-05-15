use std::fs::read;

use crate::prelude::*;

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


pub struct Context {
	pub config: AppConfig,
	pub origin: Option<String>
}

impl Context {
	pub fn new() -> Result<Self> {
		Self::with_origin(None)
	}

	pub fn with_origin(origin: Option<String>) -> Result<Self> {
		let config_file = read(CONFIG_PATH).expect("Please create a oxyconfig.json file in the root directory of your project.");
		let config = serde_json::from_slice::<AppConfigRaw>(&config_file)?;
		let config = AppConfig::from(config);
		Ok(Self { config, origin })
	}
}
