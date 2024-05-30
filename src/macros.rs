#[macro_export]
macro_rules! register_flows {
	( $( $name:ident ),* ) => {{
		let mut hash_map: HashMap<&'static str, Flow> = HashMap::new();
		$(
			hash_map.insert(stringify!($name), Flow(Box::new($name)));
		)*
		FlowController::new(hash_map)
	}};
}
