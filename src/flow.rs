use crate::{context::Context, util::*};

pub struct Flow(pub Box<dyn Fn(&Context) -> Result<()>>);

#[derive(Default)]
pub struct FlowController (HashMap<&'static str, Flow>);

impl FlowController {
	#[inline]
	pub fn new(hash_map: HashMap<&'static str, Flow>) -> Self { Self(hash_map) }

	#[inline]
	pub fn execute(&self, flow: &str, ctx: &Context) -> Result<()> { 
		self.0.get(flow).ok_or(anyhow!("Flow not found: {flow}"))?.execute(ctx)
	}
}

impl Flow {
	#[inline]
	pub fn execute(&self, ctx: &Context) -> Result<()> {
		self.0(ctx)
	}
}
