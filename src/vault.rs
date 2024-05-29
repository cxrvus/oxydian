use crate::{file::File, note::Note, util::*};
use std::fs;


pub struct Vault { root_path: PathBuf, }

impl Vault {
	pub fn new(root_path: &str) -> Result<Self> { 
		let root_path = PathBuf::from(root_path);
		if !root_path.exists() { return Err(anyhow!("Vault root path does not exist")); }
		if !root_path.is_dir() { return Err(anyhow!("Vault root path is not a directory")); }

		Ok(Self { root_path })
	}

	pub fn root_path(&self) -> &PathBuf { &self.root_path }

	pub fn path(&self, sub_path: &str) -> PathBuf { self.root_path.clone().join(sub_path) }

	pub fn get(&self, sub_path: &str) -> Result<File> { File::get(self.path(sub_path)) }

	pub fn get_note(&self, sub_path: &str) -> Result<Note> { self.get(sub_path)?.get_note() }

	pub fn ls(&self, sub_dir: &str) -> Result<Vec<File>> { self.ls_absolute(self.path(sub_dir)) }

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
