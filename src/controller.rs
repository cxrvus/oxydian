use crate::util::*;


pub trait ControllerItem {
	fn name(&self) -> String;
}

#[derive(Default)]
pub struct Controller<T: ControllerItem> (HashMap<String, T>);

impl<T: ControllerItem> Controller<T> {
	#[inline]
	pub fn new() -> Self { Self(Default::default()) }

	#[inline]
	pub fn get(&self, name: &str) -> Option<&T> { self.0.get(name) }

	pub fn register_many(mut self, items: Vec<T>) -> Result<Self> {
		for item in items { self = self.register(item)?; }
		Ok(self)
	}

	pub fn register(mut self, item: T) -> Result<Self> {
		let name = item.name();
		if self.0.contains_key(&name) { return Err(anyhow!("Item with name '{name}' already exists")); }
		else { self.0.insert(name.to_string(), item); }
		Ok(self)
	}
}
