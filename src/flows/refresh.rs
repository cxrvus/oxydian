use crate::prelude::*;


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ViewNote {
	pub view: String,
	pub locked: Option<bool>,
	pub refreshed: Option<String>,
	pub error: Option<String>,
}

pub fn refresh (ctx: &Context) -> Result<()> {
	ctx.file?.change_note(|note| {
		let props = note.get_props::<ViewNote>()?;
		if to_bool(props.locked) { return Err(anyhow!("Locked notes will not be refreshed")); }
		// let sub_flow = view_controller().execute(&props.view, vault, file)?;
		Ok(())
	})?;
	Ok(())
}

pub fn view_controller() -> FlowController {
	use super::views::test_view::test_view;
	register_flows!(test_view)
}
