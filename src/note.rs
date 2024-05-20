use serde::Deserialize;

pub struct Note
{
	content: String,
}

struct NoteSections {
	props: Option<String>,
	content: String,
}

impl NoteSections {
	fn merge(self) -> Note {
		Note { content:
			if let Some(props) = self.props {
				format!("---\n{}\n---\n{}", props, self.content)
			}
			else {
				self.content
			}
		}
	}
}

impl Note {
	pub fn new(full_content: String) -> Self { Self { content: full_content } }

	pub fn full_content(&self) -> &str { &self.content }

	pub fn get_content(&self) -> String { self.split().content }

	pub fn set_content(&mut self, new_content: String) {
		let new_content = new_content.trim().to_string() + "\n";
		let mut sections = self.split();
		sections.content = new_content;
		self.content = sections.merge().content;
	}

	pub fn get_props<'a, T: Deserialize<'a>>(&self) -> Option<T> { let _props = self.split().props; todo!("parse YAML props and turn into JSON") }

	pub fn change_props() { todo!() }

	fn split(&self) -> NoteSections {
		let re = regex::Regex::new(r"^\s*---\r?\n((?s).*?(?-s))---\r?\n((?s).*(?-s))$").unwrap();
		let caps = re.captures(&self.content);
		match caps {
			Some(caps) => NoteSections {
				props: Some(caps.get(1).unwrap().as_str().to_string()),
				content: caps.get(2).unwrap().as_str().to_string(),
			},
			None => NoteSections {
				props: None,
				content: self.content.clone(),
			},
		}
	}
}
