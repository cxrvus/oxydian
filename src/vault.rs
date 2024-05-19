use crate::{item::Item, util::*};
use std::fs;


pub struct VaultConfig {
	pub root_path: String,
}

pub struct Vault {
	root_path: PathBuf,
}

impl From<VaultConfig> for Vault {
	fn from(raw: VaultConfig) -> Self {
		Vault { root_path: PathBuf::from(raw.root_path) }
	}
}

impl Vault {
	pub fn path(&self, path: &str) -> PathBuf {
		self.root_path.clone().join(path)
	}

	pub fn ls(&self, folder: &str) -> Result<Vec<Item>> {
		let folder = self.path(folder);
		let dir = fs::read_dir(folder)?;
		let files = dir
			.filter_map(|dir_entry| {
				let dir_entry = dir_entry.ok()?;
				if !&dir_entry.file_type().ok()?.is_file() { return None; }
				let path = dir_entry.path();
				Item::get(path).ok()
			})
			.collect();

		Ok(files)
	}
}
