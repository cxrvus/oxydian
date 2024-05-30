use crate::prelude::*;

pub fn test_view(ctx: &Context) -> Result<()> {
	ctx.file?.change_note(|note| {
		note.change_content(|content| {
			content.push_str("\n\n*this note has been refreshed*\n");
			Ok(())
		})?;
		Ok(())
	})
}
