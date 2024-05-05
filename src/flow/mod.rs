use std::collections::HashMap;
mod cli;


pub struct App {
	flows: HashMap<String, Flow>,
}

impl App {
	pub fn execute() {
		let args = cli::parse();
		println!("{:?}", args);
	}

	pub fn register_flow(&mut self, name: &str, flow: Flow) {
		_ = self.flows.insert(name.to_string(), flow).expect("Flow already exists")
	}
}


type Flow = fn(Query) -> Vec<FileAction>;

pub struct Query {
	// todo
}

pub enum FileAction {
	// todo
}
