use crate::{cli::{parse_args, Command, FlowArgs}, context::Context, prelude::*};


pub struct App {
	flows: HashMap<String, Flow>
}


impl App {
	pub fn new() -> Result<Self> {
		App::with_flows(HashMap::new())
	}

	pub fn with_flows(flows: HashMap<String, Flow>) -> Result<Self> {
		Ok(App { flows })
	}

	pub fn register_flow(mut self, name: &str, flow: Flow) -> Self {
		self.flows.insert(name.to_string(), flow).expect("Flow already exists");
		return self;
	}

	pub fn execute (&self) -> Result<()> {
		let command = parse_args()?.command;
		match command {
			Command::Flow(args) => self.run_flow(args),
		}
	}

	fn run_flow(&self, args: FlowArgs) -> Result<()> {
		let origin = args.origin;
		let ctx = Context::with_origin(origin)?;
		let flow = self.flows.get(&args.flow).ok_or(anyhow!("Flow not found"))?;
		flow.execute(&ctx) // todo: turn origin into a Note
	}
}
