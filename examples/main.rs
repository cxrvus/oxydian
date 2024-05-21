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
			flows: FlowController::new()
				.register_flows(vec![
					Flow {
						name: "test_flow",
						func: FreeFn(|vault| {
							vault.flows.run("sub_flow", vault, None)?;
							Ok(())
						}),
					},
				])?,
		})?
		.execute()?
	;

	Ok(())
}
