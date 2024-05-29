#[macro_export]
macro_rules! register_flows {
	( $( $name:ident ),* ) => {{
		let mut map: HashMap<&'static str, Box<dyn Fn(&Context) -> Result<()>>> = HashMap::new();
		$(
			map.insert(stringify!($name), Box::new($name));
		)*
		map
	}};
}
