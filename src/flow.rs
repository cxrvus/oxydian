use crate::{file::File, util::*, vault::Vault};

#[derive(Default)]
pub struct FlowController {
	flows: HashMap<String, Flow>,
}

impl FlowController {
	pub fn new() -> Self { Default::default() }

	pub fn run(&self, flow: String, vault: &Vault, file_path: Option<PathBuf>) -> Result<()> {
		let flow = self.flows.get(&flow).ok_or(anyhow!("Flow not found"))?;
		let file_path = file_path.map(|file_path| { 
			if file_path.is_absolute() { file_path }
			else { vault.root_path().join(file_path) }
		});
		flow.func.execute(vault, file_path)?;
		Ok(())
	}

	pub fn register_flows(mut self, flows: Vec<Flow>) -> Result<Self> {
		for flow in flows {
			self = self.register_flow(flow)?;
		}
		Ok(self)
	}

	pub fn register_flow(mut self, flow: Flow) -> Result<Self> {
		let name = flow.name;
		if self.flows.contains_key(name) { return Err(anyhow!("Flow with name {name} already exists")); }
		else { self.flows.insert(name.to_string(), flow); }
		Ok(self)
	}

}

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
		match (self, file_path) {

			(Self::NoteFn(_) | Self::MutatingFn(_), Some(file_path)) => {
				let mut note = File::get(file_path).map_err(|_| anyhow!("Failed to get origin note file"))?;
				note.note().map_err(|e| anyhow!("Origin note is not a valid note: {}", e))?;

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
