use crate::note::Note;
use crate::util::*;
use std::fs;


pub struct File {
	path: PathBuf,
}

impl File {
	pub fn get(path: PathBuf) -> Result<Self> {
		if !path.exists() { return Err(anyhow!("File {} does not exist", path.to_str().unwrap())); }
		if !path.is_file() { return Err(anyhow!("Path {} is not a file", path.to_str().unwrap())); }

		Ok(Self { path })
	}

	pub fn path(&self) -> &PathBuf { &self.path }
	pub fn dir(&self) -> &Path { self.path.parent().unwrap() }
	pub fn name(&self) -> &str { self.path.file_name().unwrap_or_default().to_str().unwrap() }
	pub fn stem(&self) -> &str { self.path.file_stem().unwrap_or_default().to_str().unwrap() }
	pub fn ext(&self)  -> &str { self.path.extension().unwrap_or_default().to_str().unwrap() }

	pub fn to_note(&self) -> Result<Note> { Note::new(self) }

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
