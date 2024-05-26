use crate::note::Note;
use crate::util::*;
use std::fs;


pub struct File {
	path: PathBuf,
}

impl File {
	fn validate_get(path: &PathBuf) -> Result<()> {
		if !path.exists() { return Err(anyhow!("File {} does not exist", path.to_str().unwrap())); }
		if !path.is_file() { return Err(anyhow!("Path {} is not a file", path.to_str().unwrap())); }
		Ok(())
	}

	pub fn get(path: PathBuf) -> Result<Self> {
		Self::validate_get(&path)?;
		Ok(Self { path })
	}

	pub fn path(&self) -> &PathBuf { &self.path }
	pub fn dir(&self) -> &Path { self.path.parent().unwrap() }
	pub fn name(&self) -> &str { self.path.file_name().unwrap_or_default().to_str().unwrap() }
	pub fn stem(&self) -> &str { self.path.file_stem().unwrap_or_default().to_str().unwrap() }
	pub fn ext(&self)  -> &str { self.path.extension().unwrap_or_default().to_str().unwrap() }

	pub fn to_note(&self) -> Result<Note> {
		if self.ext() != "md" { return Err(anyhow!("File {} is not a markdown file", self.path().to_str().unwrap())); }
		let content = fs::read_to_string(self.path())?;
		Ok(Note::new(content))
	}

	pub fn write(&self, content: String) -> Result<()> {
		fs::write(self.path(), content).map_err(|e| anyhow!("Could not write to file:\n{}\n{:?}", e, self.path()))
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
