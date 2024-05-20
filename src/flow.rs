use crate::{item::Item, util::*, vault::Vault};


pub type FlowMap = HashMap<String, Flow>;
pub type FlowList = Vec<Flow>;

pub struct Flow {
	pub name: &'static str,
	pub func: FlowFn,
}

pub enum FlowFn {
	FreeFn (fn(&Vault) -> Result<()>),
	NoteFn (fn(&Vault, &Item) -> Result<()>),
	MutatingFn (fn(&Vault, &mut Item) -> Result<()>),
}

impl FlowFn {
	pub fn execute(&self, config: &Vault, note_path: Option<PathBuf>) -> Result<()> {
		match (self, note_path) {

			(Self::NoteFn(_) | Self::MutatingFn(_), Some(note_path)) => {
				let mut note = Item::get(note_path).map_err(|_| anyhow!("Failed to get origin note file"))?;
				note.note().map_err(|e| anyhow!("Origin note is not a valid note: {}", e))?;

				match self {
					Self::NoteFn(flow) 		=> flow(config, &note),
					Self::MutatingFn(flow) 	=> flow(config, &mut note),
					_ 						=> unreachable!(),
				}
			} 

			(Self::FreeFn(flow), None) 					=> flow(config),

			(Self::NoteFn(_), None) 	=> Err(anyhow!("Note flows require an origin note")),
			(Self::MutatingFn(_), None) => Err(anyhow!("Mutating note flows require an origin note")),
			(Self::FreeFn(_), Some(_)) 	=> Err(anyhow!("FreeFlows do not accept an origin note")),

		}
	}
}
