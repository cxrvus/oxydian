use crate::{item::Item, note::Note, util::*, vault_config::VaultConfig};


pub enum IFlow {
	Flow (fn(&VaultConfig, &Note) -> Result<()>),
	FreeFlow (fn(&VaultConfig) -> Result<()>),
}

impl IFlow {
	pub fn execute(&self, config: &VaultConfig, note: Option<PathBuf>) -> Result<()> {
		match (self, note) {
			(Self::Flow(flow), Some(note_path)) => {
				let note_item = Item::get(&note_path).map_err(|_| anyhow!("Failed to get origin note file"))?;
				let note = note_item.note().ok_or_else(|| anyhow!("Origin note is not a markdown file"))?;
				flow(config, &note)
			}
			(Self::FreeFlow(flow), None) => flow(config),
			(Self::FreeFlow(_), Some(_)) => Err(anyhow!("FreeFlows do not accept origins")),
			(Self::Flow(_), None) => Err(anyhow!("Flows require an origin")),
		}
	}
}
