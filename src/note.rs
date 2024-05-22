use std::fs;
use crate::{file::File, util::*};


pub struct Note<'n>
{
	content: String,
	file: &'n File,
}

pub struct NoteSections<'s> {
	pub props: Option<&'s str>,
	pub content: &'s str,
}

impl<'s> NoteSections<'s> {
	fn merge(self) -> String {
		if let Some(props) = self.props { format!("---\n{}\n---\n{}", props, self.content) }
		else { self.content.to_string() }
	}
}

impl<'n> Note<'n> {
	pub fn new(file: &'n File) -> Result<Self> {
		if file.ext() != "md" { return Err(anyhow!("File {} is not a markdown file", file.path().to_str().unwrap())); }
		let content = fs::read_to_string(file.path())?;
		Ok(Self { file, content })
	}

	pub fn get_full_content(&self) -> &str { &self.content }

	pub fn get_content(&self) -> &str { self.split().content }

	pub fn get_props<'de, T>(&'de self) -> Result<T> 
		where T: Deserialize<'de>
	{ 
		let props = self.split().props.ok_or(anyhow!("No properties found"))?;
		serde_yaml::from_str::<T>(props).map_err(|e| anyhow!("Could not parse properties:\n{}", e))
	}

	pub fn change_content(&mut self, func: fn(&mut String) -> Result<()>) -> Result<()> {
		let mut content = self.get_content().to_string();
		func(&mut content)?;
		self.content = NoteSections { content: &content, ..self.split() }.merge();
		Ok(())
	}

	pub fn change_props<'de, T, U>(&'de mut self, func: fn(&mut Result<T>) -> Result<U>) -> Result<()> 
		where T: Deserialize<'de>, U: Serialize
	{
		let mut props = self.get_props();
		func(&mut props)?;
		// todo: fix
		// if let Ok(props) = props {
		// 	let props = serde_yaml::to_string(&props)?;
		// 	self.content = NoteSections { props: Some(&props), ..self.split() }.merge();
		// }
		Ok(())
	}

	pub fn write(self) -> Result<()> {
		fs::write(self.file.path(), self.content).map_err(|e| anyhow!("Could not write to note:\n{}\n{:?}", e, self.file.path()))
	}

	fn split(&self) -> NoteSections {
		let re = regex::Regex::new(r"^\s*---\r?\n((?s).*?(?-s))---\r?\n((?s).*(?-s))$").unwrap();
		let caps = re.captures(&self.content);
		match caps {
			Some(caps) => NoteSections {
				props: Some(caps.get(1).unwrap().as_str()),
				content: caps.get(2).unwrap().as_str(),
			},
			None => NoteSections {
				props: None,
				content: &self.content,
			},
		}
	}
}
