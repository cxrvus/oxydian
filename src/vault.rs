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
	pub fn root_path(&self) -> &PathBuf {
		&self.root_path
	}

	pub fn path(&self, sub_path: &str) -> PathBuf {
		self.root_path.clone().join(sub_path)
	}

	pub fn get(&self, sub_path: &str) -> Result<Item> {
		Item::get(self.path(sub_path))
	}

	pub fn ls(&self, sub_dir: &str) -> Result<Vec<Item>> {
		self.ls_absolute(self.path(sub_dir))
	}

	pub fn ls_absolute(&self, folder: PathBuf) -> Result<Vec<Item>> {
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
