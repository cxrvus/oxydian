use serde::Deserialize;

pub struct Note
{
	content: String,
}

struct NoteSections {
	content: String,
	props: Option<String>,
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
		let full_content = self.content.clone();
		let mut sections = full_content.split("---\n");

		let first = sections.next().expect("impossible: split always returns at least one element");
		let second = sections.next();
		let has_props = first.is_empty() && second.is_some();

		if has_props {
			let props = second.expect("impossible: checked in has_props").to_owned();
			let content = sections.collect::<String>();
			NoteSections { content, props: Some(props) }
		}
		else {
			NoteSections { content: full_content, props: None }
		}
	}

}
