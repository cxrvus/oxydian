use crate::{controller::ControllerItem, file::File, util::*, vault::Vault};


pub struct Flow {
	pub name: &'static str,
	pub func: FlowFn,
}

pub enum FlowFn {
	FreeFn (fn(&Vault) -> Result<()>),
	NoteFn (fn(&Vault, &File) -> Result<()>),
	MutatingFn (fn(&Vault, &mut File) -> Result<()>),
}

impl FlowFn {
	pub fn execute(&self, vault: &Vault, file_path: Option<PathBuf>) -> Result<()> {
		let file_path = file_path.map(|file_path| { 
			if file_path.is_absolute() { file_path }
			else { vault.root_path().join(file_path) }
		});

		match (self, file_path) {

			(Self::NoteFn(_) | Self::MutatingFn(_), Some(file_path)) => {
				let mut note = File::get(file_path).map_err(|_| anyhow!("Failed to get origin note file"))?;
				note.to_note().map_err(|e| anyhow!("Origin note is not a valid note: {}", e))?;

				match self {
					Self::NoteFn(flow) 		=> flow(vault, &note),
					Self::MutatingFn(flow) 	=> flow(vault, &mut note),
					_ 						=> unreachable!(),
				}
			} 

			(Self::FreeFn(flow), None) 					=> flow(vault),

			(Self::NoteFn(_), None) 	=> Err(anyhow!("Note flows require an origin note")),
			(Self::MutatingFn(_), None) => Err(anyhow!("Mutating note flows require an origin note")),
			(Self::FreeFn(_), Some(_)) 	=> Err(anyhow!("FreeFlows do not accept an origin note")),

		}
	}
}

impl ControllerItem for Flow {
	#[inline]
	fn name(&self) -> String { self.name.to_string() }
}
