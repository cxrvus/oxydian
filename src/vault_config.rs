use crate::util::*;


pub struct VaultSetup {
	pub root_path: String,
}

pub struct VaultConfig {
	root_path: PathBuf,
}

impl From<VaultSetup> for VaultConfig {
	fn from(raw: VaultSetup) -> Self {
		VaultConfig { root_path: PathBuf::from(raw.root_path) }
	}
}

impl VaultConfig {
	pub fn path(&self, path: &str) -> PathBuf {
		self.root_path.clone().join(path)
	}
}
