use serde::{Deserialize, Serialize};

pub struct Note
{
	content: String,
}

pub struct NoteSections<'a> {
	pub props: Option<&'a str>,
	pub content: &'a str,
}

impl<'a> NoteSections<'a> {
	fn merge(self) -> Note {
		Note { content:
			if let Some(props) = self.props {
				format!("---\n{}\n---\n{}", props, self.content)
			}
			else {
				self.content.to_string()
			}
		}
	}
}

impl Note {
	pub fn new(full_content: String) -> Self { Self { content: full_content } }

	pub fn full_content(&self) -> &str { &self.content }

	pub fn get_content(&self) -> &str { self.split().content }

	pub fn set_content(&mut self, new_content: &str) {
		let new_content = new_content.trim();
		let mut sections = self.split();
		sections.content = new_content;
		self.content = sections.merge().content;
	}

	pub fn get_props<'a, T: Deserialize<'a> + Serialize>(&'a self) -> Option<T> { 
		let props = self.split().props?;
		serde_yaml::from_str::<T>(props).ok()
	}

	pub fn change<'a, T: Deserialize<'a> + Serialize>(&'a mut self, func: fn(&mut NoteSections) -> ()) {
		let mut sections = self.split();
		func(&mut sections);
		self.content = sections.merge().content;
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
				content: self.content.as_str(),
			},
		}
	}
}
