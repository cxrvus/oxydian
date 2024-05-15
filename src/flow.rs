use crate::{file::Note, prelude::*};

pub struct Flow (fn(&Context, origin: Option<Note>) -> Result<()>);

impl Flow {
	pub fn execute(&self, ctx: &Context, origin: Option<Note>) -> Result<()> {
		(self.0)(ctx, origin)
	}
}
