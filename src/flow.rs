use crate::{context::Context, prelude::*};

pub struct Flow (fn(&Context) -> Result<()>);

impl Flow {
	pub fn execute(&self, ctx: &Context) -> Result<()> {
		(self.0)(ctx)
	}
}
