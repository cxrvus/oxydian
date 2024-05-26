pub use std::{collections::HashMap, path::{Path, PathBuf}};
pub use anyhow::{anyhow, Context, Result};
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[inline]
pub fn to_bool(bool_option: Option<bool>) -> bool {
	match bool_option {
		Some(bool) => bool,
		None => false,
	}
}

