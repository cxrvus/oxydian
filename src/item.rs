use crate::note::Note;
use std::{fs, path::{Path, PathBuf}};
use anyhow::{anyhow, Result};


pub struct Item {
	path: PathBuf,
}

impl Item {
	pub fn get(path: PathBuf) -> Result<Self> {
		if !path.exists() { return Err(anyhow!("File does not exist")); }
		if !path.is_file() { return Err(anyhow!("Path is not a file")); }

		Ok(Self { path })
	}

	pub fn path(&self) -> &PathBuf { &self.path }
	pub fn dir(&self) -> &Path { self.path.parent().unwrap() }
	pub fn name(&self) -> &str { self.path.file_name().unwrap_or_default().to_str().unwrap() }
	pub fn stem(&self) -> &str { self.path.file_stem().unwrap_or_default().to_str().unwrap() }
	pub fn ext(&self)  -> &str { self.path.extension().unwrap_or_default().to_str().unwrap() }

	pub fn note(&self) -> Option<Note> {
		if self.ext() == "md" {
			let content = fs::read_to_string(self.path()).expect("Failed to read note's content");
			Some(Note::new(content))
		}
		else { None }
	}

	pub fn rm(self) -> Result<()> {
		fs::remove_file(self.path())?;
		Ok(())
	}

	pub fn mv(&mut self, target_dir: &Path) -> Result<()> {
		if !target_dir.is_dir() { return Err(anyhow!("Target needs to be a directory")); }

		let new_path = target_dir.join(self.name());
		fs::rename(self.path(), &new_path)?;

		self.path = new_path;
		Ok(())
	}
}
