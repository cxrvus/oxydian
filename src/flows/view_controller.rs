use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ViewNote {
	pub view: String,
	pub refreshed: Option<String>,
	pub error: Option<String>,
}
