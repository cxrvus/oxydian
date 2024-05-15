use crate::{note::Note, util::*, vault_config::VaultConfig};

pub enum IFlow {
	Flow (fn(&VaultConfig, &Note) -> Result<()>),
	FreeFlow (fn(&VaultConfig) -> Result<()>),
}

impl IFlow {
	pub fn execute(&self, config: &VaultConfig, origin: Option<String>) -> Result<()> {
		match (self, origin) {
			(Self::FreeFlow(flow), None) => flow(config),
			(Self::FreeFlow(flow), Some(_)) => Err(anyhow!("FreeFlows do not accept origins")),
			(Self::Flow(flow), Some(origin)) => flow(config, todo!("load note from origin")),
			(Self::Flow(_), None) => Err(anyhow!("Flows require an origin")),
		}
	}
}
