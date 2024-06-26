use crate::link::*;



pub struct View(pub Vec<Line>);

#[derive(Default)]
pub enum Token {
	#[default]
	Empty,
	Break,
	Text(String),
	Tag(String),
	Link(Link),
	ExtLink(ExtLink),
}

#[derive(Default)]
pub enum Line {
	#[default]
	Empty,
	Break,
	Text(Vec<Token>),
	Paragraph(Vec<Token>),
	Heading(u8, Vec<Token>),
	Bullet(u8, Vec<Token>),
	Check(u8, bool, Vec<Token>),
}

impl From<View> for String {
	fn from(view: View) -> Self {
		view.0.into_iter().map(|line| line.into_string()).collect()
	}
}

impl Line {
	pub fn into_tokens(self) -> Vec<Token> {
		self.into()
	}
	pub fn into_string(self) -> String {
		self.into_tokens().into_iter().map(|token| token.into_string()).collect()
	}
}

fn linebreak(tokens: Vec<Token>) -> Vec<Token> {
	tokens.into_iter().chain(vec![Token::Break]).collect()
}

impl From<Line> for Vec<Token> {
	fn from(line: Line) -> Self {
		use Line::*;
		let tokens = match line {
			Empty => vec![],
			Break => linebreak(vec![]),
			Text(tokens) => tokens,
			Paragraph(tokens) => linebreak(tokens),
			Heading(_, tokens) =>
				vec![Token::Text("# ".into())].into_iter()
				.chain(tokens)
				.chain(vec![Token::Break])
				.collect()
			,
			Bullet(_, _) => todo!(),
			Check(_, _, _) => todo!(),
		};
		linebreak(tokens)
	}

}

impl Token {
	fn into_string(self) -> String {
		self.into()
	}
}

impl From<Token> for String {
	fn from(token: Token) -> Self {
		use Token::*;
		match token {
			Empty => "".into(),
			Break => "\n".into(),
			Text(text) => text,
			Tag(tag) => format!("#{tag}"),
			Link(link) => link.into(),
			ExtLink(link) => link.into(),
		}
	}
}
