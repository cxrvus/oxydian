use crate::prelude::*;
use super::views;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ViewNote {
	pub view: String,
	pub locked: Option<bool>,
	pub refreshed: Option<String>,
	pub error: Option<String>,
}

pub const REFRESH: Flow = Flow {
	name: "refresh",
	func: FlowFn::NoteFn(|_vault, file| {
		file.change_note(|note| {
			let props = note.get_props::<ViewNote>()?;
			if to_bool(props.locked) { return Err(anyhow!("Locked notes will not be refreshed")); }
			// todo: fix having flows as non-capturing `fn` closures
			// let sub_flow = view_controller().execute(&props.view, vault, file)?;
			Ok(())
		})?;
		Ok(())
	})
};

pub fn view_controller() -> FlowController {
	FlowController::new()
		.register_many(vec![
			views::test_view::TEST_VIEW
		])
		.unwrap()
}
