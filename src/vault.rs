use crate::{controller::Controller, file::File, flow::Flow, util::*};
use std::fs;


pub struct Vault {
	pub root_path: PathBuf,
	pub sub_paths: SubPaths,
	pub flows: Controller<Flow>,
}

pub struct SubPaths {
	pub notes: Option<PathBuf>,
}

impl Vault {
	pub fn root_path(&self) -> &PathBuf {
		&self.root_path
	}

	pub fn path(&self, sub_path: &str) -> PathBuf {
		self.root_path.clone().join(sub_path)
	}

	pub fn get(&self, sub_path: &str) -> Result<File> {
		File::get(self.path(sub_path))
	}

	pub fn ls(&self, sub_dir: &str) -> Result<Vec<File>> {
		self.ls_absolute(self.path(sub_dir))
	}

	pub fn ls_absolute(&self, folder: PathBuf) -> Result<Vec<File>> {
		let dir = fs::read_dir(folder)?;
		let files = dir
			.filter_map(|dir_entry| {
				let dir_entry = dir_entry.ok()?;
				if !&dir_entry.file_type().ok()?.is_file() { return None; }
				let path = dir_entry.path();
				File::get(path).ok()
			})
			.collect();

		Ok(files)
	}
}
