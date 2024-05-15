use std::fs::read;
use crate::util::*;


#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
struct VaultConfigRaw {
	root_path: String,
}

#[derive(Clone)]
pub struct VaultConfig {
	root_path: PathBuf,
}

impl From<VaultConfigRaw> for VaultConfig {
	fn from(raw: VaultConfigRaw) -> Self {
		VaultConfig { root_path: PathBuf::from(raw.root_path) }
	}
}

impl VaultConfig {
	pub fn load() -> Result<Self> {
		let raw_config_file = read(CONFIG_PATH).expect("Please create a oxyconfig.json file in the root directory of your project.");
		let raw_config = serde_json::from_slice::<VaultConfigRaw>(&raw_config_file)?;
		Ok(raw_config.into())
	}
}

const CONFIG_PATH: &str = "./oxydian/config.json";
