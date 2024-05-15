use std::fs::{read, read_dir};

use crate::{item::Item, prelude::*};

const CONFIG_PATH: &str = "./oxydian/config.json";


#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
struct AppConfigRaw {
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

	pub fn ls(&self) -> Result<Vec<Item>> {
		Self::ls_internal(&self.config.vault_path)
	}

	pub fn ls_external(&self, folder: &str) -> Result<Vec<Item>> {
		Self::ls_internal(&PathBuf::from(folder))
	}

	fn ls_internal(folder: &PathBuf) -> Result<Vec<Item>> {
		let dir = read_dir(folder)?;
		let files = dir
			.filter_map(|dir_entry| {
				let dir_entry = dir_entry.ok()?;
				if !&dir_entry.file_type().ok()?.is_file() { return None; }
				let path = dir_entry.path();

				let id = path.file_stem()?.to_str()?.to_string();
				let extension = path.extension()?.to_str()?.to_string();

				if !Item::ALLOWED_EXTENSIONS.contains(&extension.as_str()) { return None; }

				Some(Item::new(&path, &id).ok()?)
			})
			.collect();

		Ok(files)
	}
}
