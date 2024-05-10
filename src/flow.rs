use crate::prelude::*;
use crate::fs_util;

pub struct Flow {
	pub name: String,
	pub folder: String,
	// function: fn(Vec<Props>) -> Result<Vec<FileAction>>,
}

impl Flow {
	pub fn invoke(&self) -> Result<Vec<FileAction>> {
		let props = fs_util::read_files(&self.folder)?;
		todo!("invoke flow using correct props type")
		// return (self.function)(props)
	}
}

pub enum FileAction {
	// todo
}
