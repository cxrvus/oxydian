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
					println!("content:\n{}\n", file.get_note()?.get_content());
					println!("props:\n{:?}", file.get_note()?.get_props::<NoteBase>());
					Ok(())
				}),
			},
		])?,
	}.execute()?;

	Ok(())
}
