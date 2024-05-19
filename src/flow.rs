use crate::{item::Item, note::Note, util::*, vault::Vault};


pub type FlowMap = HashMap<String, Flow>;
pub type FlowList = Vec<Flow>;

pub struct Flow {
	pub name: &'static str,
	pub func: FlowFn,
}

pub enum FlowFn {
	FreeFn (fn(&Vault) -> Result<()>),
	NoteFn (fn(&Vault, &Note) -> Result<()>),
	MutatingFn (fn(&Vault, &mut Note) -> Result<()>),
}

impl FlowFn {
	pub fn execute(&self, config: &Vault, note_path: Option<PathBuf>) -> Result<()> {
		match (self, note_path) {
			(Self::FreeFn(flow), None) 				=> flow(config),
			(Self::NoteFn(flow), Some(note_path)) 	=> flow(config, &Self::get_origin_note(note_path)?),
			(Self::MutatingFn(flow), Some(note_path)) => flow(config, &mut Self::get_origin_note(note_path)?),

			(Self::FreeFn(_), Some(_)) 	=> Err(anyhow!("FreeFlows do not accept an origin note")),
			(Self::NoteFn(_), None) 		=> Err(anyhow!("Note flows require an origin note")),
			(Self::MutatingFn(_), None) 	=> Err(anyhow!("Mutating note flows require an origin note")),

		}
	}

	fn get_origin_note(note_path: PathBuf) -> Result<Note> {
		let note_item = Item::get(note_path).map_err(|_| anyhow!("Failed to get origin note file"))?;
		let note = note_item.note().ok_or_else(|| anyhow!("Origin note is not a markdown file"))?;
		Ok(note)
	}
}
