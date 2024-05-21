use oxydian::prelude::*;

fn main() {
	if let Err(e) = execute() { eprintln!("<!>\n{}", e); }
}

fn execute() -> Result<()> {
	App::new(VaultConfig {
			root_path:  "/home/cxrvus/Obsidian/TestVault".into()
		})?
		.with_flows(flows::ViewController {
			notes_path: "/home/cxrvus/Obsidian/TestVault/notes".into(),
			views: Default::default(),
		}.flows())?
		.execute()?
	;

	Ok(())
}
