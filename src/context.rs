use crate::prelude::*;
use crate::app::AppConfig;

pub struct Context {
	pub config: AppConfig,
	pub origin: Option<String>
}

impl Context {
	pub fn new(config: AppConfig, origin: Option<String>) -> Self {
		Context { config, origin }
	}
}
