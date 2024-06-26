pub use std::{collections::HashMap, path::{Path, PathBuf}};
pub use anyhow::{anyhow, Context as AnyhowContext, Result};
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[inline]
pub fn to_bool(bool_option: Option<bool>) -> bool {
	bool_option.unwrap_or(false)
}

