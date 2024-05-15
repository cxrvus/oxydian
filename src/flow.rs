use crate::{app::AppContext, prelude::*};

pub struct Flow (fn(&AppContext) -> Result<()>);

impl Flow {
	pub fn execute(&self, ctx: &AppContext) -> Result<()> {
		(self.0)(ctx)
	}
}
