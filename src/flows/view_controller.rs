use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ViewNote {
	pub view: String,
	pub refreshed: Option<String>,
	pub error: Option<String>,
}

pub struct ViewController {
	pub notes_path: PathBuf,
	pub views: Vec<View>,
}

impl ViewController {
	pub fn flows(self) -> Vec<Flow> {
		vec![
			Flow { 
				name: "refresh",
				func: todo!()
			},
			Flow {
				name: "refresh_all",
				func: todo!()
			}
		]
	}
}
