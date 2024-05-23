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
				.register_many(vec![
					Flow {
						name: "test_flow",
						func: FreeFn(|vault| {
							let sub_flow = vault.flows.get("sub_flow").ok_or(anyhow!("sub_flow not found"))?;
							sub_flow.func.execute(vault, None)
						}),
					},
				])?,
		})?
		.execute()?
	;

	Ok(())
}
