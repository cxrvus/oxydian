pub struct Link{
	path: String,
	alias: Option<String>,
	sub: Option<String>,
	embed: bool,
}

pub struct ExtLink{
	url: String,
	alias: String,
	embed: bool,
}

impl From<Link> for String {
	fn from(link: Link) -> Self {
		let Link { path, alias, sub, embed } = link;
		let embed = if embed { "!" } else { "" };
		let alias = alias.map(|x| format!("|{}", x)).unwrap_or_default();
		let sub   =   sub.map(|x| format!("#{}", x)).unwrap_or_default();
		format!("{embed}[[{path}{sub}{alias}]]")
	}
}

impl From<ExtLink> for String {
	fn from(link: ExtLink) -> Self {
		let ExtLink { url, alias, embed } = link;
		let embed = if embed { "!" } else { "" };
		let alias = if !alias.is_empty() { format!("[{alias}]") } else { "[Link]".into() };
		let url = format!("({url})");
		format!("{embed}{alias}{url}")
	}
}
