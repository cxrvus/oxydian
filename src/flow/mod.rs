use std::{collections::HashMap};
use anyhow::{anyhow, Result};
use serde::{Serialize, Deserialize};
mod cli;
mod fs_util;

pub struct App<Props: Serialize> {
	flows: HashMap<String, Flow<Props>>
}

impl<Props: Serialize> App<Props> {
	pub fn execute(&self) {
		let actions = self.get_actions();
		match actions {
			Ok(actions) => { self.invoke_actions(actions) },
			Err(e) => { println!("Error: {}", e.to_string()) },
		}
	}

	fn get_actions (&self) -> Result<Vec<FileAction>> {
		let args = cli::parse()?;
		let flow = self.flows.get(&args.flow).ok_or(anyhow!("Flow not found"))?;
		let actions = flow.invoke()?;
		Ok(actions)
	}

	fn invoke_actions (&self, actions: Vec<FileAction>) {
		todo!("invoke actions")
	}

	pub fn register_flow(mut self, name: &str, flow: Flow<Props>) -> Self {
		self.flows.insert(name.to_string(), flow).expect("Flow already exists");
		return self;
	}
}

pub struct Flow<Props> {
	pub name: String,
	pub folder: String,
	function: fn(Vec<Props>) -> Result<Vec<FileAction>>,
}

impl<Props> Flow<Props> {
	pub fn invoke(&self) -> Result<Vec<FileAction>> {
		let props = fs_util::read_files(&self.folder)?;
		todo!("invoke flow using correct props type")
		// return (self.function)(props)
	}
}

pub enum FileAction {
	// todo
}
