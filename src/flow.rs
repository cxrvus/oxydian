use crate::{note::Note, util::*, vault_config::VaultConfig};


pub enum IFlow {
	Flow (fn(&VaultConfig, &Note) -> Result<()>),
	FreeFlow (fn(&VaultConfig) -> Result<()>),
}

impl IFlow {
	pub fn execute(&self, config: &VaultConfig, origin: Option<String>) -> Result<()> {
		match (self, origin) {
			(Self::FreeFlow(flow), None) => flow(config),
			(Self::FreeFlow(_), Some(_)) => Err(anyhow!("FreeFlows do not accept origins")),
			(Self::Flow(_flow), Some(_origin)) => todo!("load note from origin and return a flow"),
			(Self::Flow(_), None) => Err(anyhow!("Flows require an origin")),
		}
	}
}
