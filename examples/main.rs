use oxydian::prelude::*;


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
				func: FreeFn(|_| {
					println!("Hello World!");
					Ok(())
				}),
			},
		])?,
	};

	Ok(())
}
