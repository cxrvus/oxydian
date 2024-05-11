use std::{fs, path::{Path, PathBuf}};
use anyhow::{anyhow, Result};
// use serde::{Deserialize, Serialize};

pub struct VaultFile {
	path: PathBuf,
	sub_path: String,
}

impl VaultFile {
	pub fn new(vault_dir: &PathBuf, sub_path: &str) -> Result<Self> {
		let path = vault_dir.join(PathBuf::from(sub_path));
		if !path.exists() { return Err(anyhow!("File does not exist")); }
		if !path.is_file() { return Err(anyhow!("Path is not a file")); }

		Ok(VaultFile { path: PathBuf::from(path), sub_path: sub_path.into() })
	}

	pub fn path(&self) -> &Path { self.path.as_path() }
	pub fn sub_path(&self) -> &String { &self.sub_path }
	pub fn name(&self) -> &str { self.path.file_name().unwrap().to_str().unwrap() }
	pub fn stem(&self) -> &str { self.path.file_stem().unwrap().to_str().unwrap() }
	pub fn ext(&self) -> &str { self.path.extension().unwrap().to_str().unwrap() }
	pub fn dir(&self) -> &Path { self.path.parent().unwrap() }

	pub fn note(&self) -> Option<Note> {
		if self.ext() == "md" {
			let content = fs::read_to_string(self.path()).expect("Failed to read note's content");
			Some(Note::new(content))
		}
		else { None }
	}

	pub fn rm(self, file: VaultFile) -> Result<()> {
		fs::remove_file(file.path())?;
		Ok(())
	}

	pub fn mv(&mut self, file: VaultFile, target_dir: &PathBuf) -> Result<()> {
		if !target_dir.is_dir() { return Err(anyhow!("Target needs to be a directory")); }

		let new_path = target_dir.join(file.name());
		fs::rename(file.path(), &new_path)?;

		self.path = new_path;
		Ok(())
	}
}

pub struct Note
{
	content: String,
}

struct NoteSections {
	content: String,
	props: Option<String>,
}

impl NoteSections {
	fn merge(self) -> Note {
		Note { content:
			if let Some(props) = self.props {
				format!("---\n{}\n---\n{}", props, self.content)
			}
			else {
				self.content
			}
		}
	}
}

impl Note {
	fn split(&self) -> NoteSections {
		let full_content = self.content.clone();
		let mut sections = full_content.split("---\n");

		let first = sections.next().expect("impossible: split always returns at least one element");
		let second = sections.next();
		let has_props = first == "" && second.is_some();

		if has_props {
			let props = second.expect("impossible: checked in has_props").to_owned();
			let content = sections.collect::<String>();
			NoteSections { content, props: Some(props) }
		}
		else {
			NoteSections { content: full_content, props: None }
		}
	}

	pub fn new(full_content: String) -> Self { Self { content: full_content } }

	pub fn full_content(&self) -> &str { &self.content }

	pub fn get_content(&self) -> String { self.split().content }

	pub fn set_content(&mut self, new_content: String) {
		let new_content = new_content.trim().to_string() + "\n";
		let mut sections = self.split();
		sections.content = new_content;
		self.content = sections.merge().content;
	}

	pub fn get_props(&self) -> Option<String> { self.split().props; todo!("parse YAML props and turn into JSON") }

	pub fn change_props() { todo!() }
}
