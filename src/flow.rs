use crate::{item::Item, note::Note, util::*, vault_config::VaultConfig};


pub enum Flow {
	Free (fn(&VaultConfig) -> Result<()>),
	Note (fn(&VaultConfig, &Note) -> Result<()>),
	Mutating (fn(&VaultConfig, &mut Note) -> Result<()>),
}

impl Flow {
	pub fn execute(&self, config: &VaultConfig, note_path: Option<PathBuf>) -> Result<()> {
		match (self, note_path) {
			(Self::Free(flow), None) 				=> flow(config),
			(Self::Note(flow), Some(note_path)) 	=> flow(config, &Self::get_origin_note(note_path)?),
			(Self::Mutating(flow), Some(note_path)) => flow(config, &mut Self::get_origin_note(note_path)?),

			(Self::Free(_), Some(_)) 	=> Err(anyhow!("FreeFlows do not accept an origin note")),
			(Self::Note(_), None) 		=> Err(anyhow!("Note flows require an origin note")),
			(Self::Mutating(_), None) 	=> Err(anyhow!("Mutating note flows require an origin note")),

		}
	}

	fn get_origin_note(note_path: PathBuf) -> Result<Note> {
		let note_item = Item::get(&note_path).map_err(|_| anyhow!("Failed to get origin note file"))?;
		let note = note_item.note().ok_or_else(|| anyhow!("Origin note is not a markdown file"))?;
		Ok(note)
	}
}
