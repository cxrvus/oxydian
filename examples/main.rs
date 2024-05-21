use oxydian::prelude::*;

fn main() {
	if let Err(e) = execute() { eprintln!("<!>\n{}", e); }
}

fn execute() -> Result<()> {
	App::new(Vault {
			root_path: "/home/cxrvus/Obsidian/TestVault".into(),
			sub_paths: SubPaths {
				notes: Some("Notes".into()),
			},
		})?
		.with_flows(flows::ViewController {
			notes_path: "Notes".into(),
			views: Default::default(),
		}.flows())?
		.execute()?
	;

	Ok(())
}
