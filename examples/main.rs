use oxydian::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct NoteBase {
	pub of: Vec<String>,
}

fn main() {
	if let Err(e) = execute() { eprintln!("<!>\n{}", e); }
}

fn execute() -> Result<()> {
	App {
		vault: Vault {
			root_path: "/home/cxrvus/Obsidian/TestVault".into(),
			sub_paths: SubPaths {
				notes: Some("Notes".into()),
			},
		},
		flows: FlowController::new().register_many(vec![
			Flow {
				name: "test_flow",
				func: NoteFn(|_, file| {
					file.change_note(|note| {
						note.change_content(|content| {
							content.push_str("\n\n*this note has been refreshed*\n");
							Ok(())
						})?
						.change_props::<NoteBase>(|props| {
							props.of.push("test_flow".into());
							Ok(())
						})?;
						Ok(())
					})?;
					Ok(())
				}),
			},
		])?,
	}.execute()?;

	Ok(())
}
