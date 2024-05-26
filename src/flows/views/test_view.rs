use crate::prelude::*;

pub const TEST_VIEW: Flow = Flow {
	name: "test_view",
	func: NoteFn(|_, file| {
		file.change_note(|note| {
			note.change_content(|content| {
				content.push_str("\n\n*this note has been refreshed*\n");
				Ok(())
			})
		})
	})
};
