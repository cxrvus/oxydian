use crate::{file::File, util::*, vault::Vault};


pub struct FlowController (HashMap<String, Flow>);

pub struct Flow {
	pub name: &'static str,
	pub func: FlowFn,
}

pub enum FlowFn {
	FreeFn (fn(&Vault) -> Result<()>),
	NoteFn (fn(&Vault, &mut File) -> Result<()>),
}

impl Default for FlowController {
    fn default() -> Self {
        Self::new()
    }
}

impl FlowController {
	#[inline]
	pub fn new() -> Self { Self(Default::default()) }

	#[inline]
	pub fn execute(&self, flow: &str, vault: &Vault, file: Option<File> ) -> Result<()> { 
		self.0.get(flow).ok_or(anyhow!("Flow not found: {flow}"))?.execute(vault, file)
	}

	pub fn register_many(mut self, flows: Vec<Flow>) -> Result<Self> {
		for flow in flows { self = self.register(flow)?; }
		Ok(self)
	}

	pub fn register(mut self, flow: Flow) -> Result<Self> {
		let name = flow.name;
		if self.0.contains_key(name) { return Err(anyhow!("Flow with name '{name}' already exists")); }
		else { self.0.insert(name.to_string(), flow); }
		Ok(self)
	}
}

impl Flow {
	pub fn execute(&self, vault: &Vault, file: Option<File>) -> Result<()> {
		use FlowFn::*;
		match (&self.func, file) {
			(FreeFn(flow), None) 			=> flow(vault),
			(NoteFn(flow), Some(mut file)) 	=> flow(vault, &mut file),
			(NoteFn(_), None) 				=> Err(anyhow!("Note flows require an origin note")),
			(FreeFn(_), Some(_)) 			=> Err(anyhow!("FreeFlows do not accept an origin note")),
		}
	}
}
