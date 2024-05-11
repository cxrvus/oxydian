use crate::{file::Note, prelude::*};

pub struct Flow (fn(&Vault, origin: Option<Note>) -> Result<()>);

impl Flow {
	pub fn execute(&self, vault: &Vault, origin: Option<Note>) -> Result<()> {
		(self.0)(vault, origin)
	}
}
