use crate::util::*;


pub struct Note { content: String, }

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

impl Note {
	pub fn new(content: String) -> Self { Self { content } }

	pub fn get_full_content(&self) -> &str { &self.content }

	pub fn get_content(&self) -> &str { self.split().content }

	pub fn get_props<T>(&self) -> Result<T> 
		where T: DeserializeOwned
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

	pub fn change_props<T>(&mut self, func: fn(&mut T) -> Result<()>) -> Result<()> 
		where T: DeserializeOwned + Serialize
	{
		let mut props = self.get_props::<T>()?;
		func(&mut props)?;
		let props_str = serde_yaml::to_string(&props)?;
		self.content = NoteSections { props: Some(&props_str), ..self.split() }.merge();
		Ok(())
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
