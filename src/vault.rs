use std::{fs, path::PathBuf};

use crate::prelude::*;
use crate::app::AppConfig;

pub struct Vault {
	pub vault_path: PathBuf,
	pub origin: Option<String>
}

impl Vault {
	pub fn new(vault_path: PathBuf, origin: Option<String>) -> Self {
		Vault { vault_path, origin }
	}
}
