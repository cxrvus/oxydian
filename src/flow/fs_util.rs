use anyhow::{anyhow, Result};
use serde::{Serialize, Deserialize};
use std::fs;
#[derive(Serialize, Deserialize)]
pub struct FileProps {
	pub id: String,
	pub extension: String,
}

const ALLOWED_EXTENSIONS: [&str; 11] = ["md", "pdf", "jpg", "jpeg", "png", "webp", "svg", "mp4", "mp3", "ogg", "wav"];

pub fn read_files(folder: &str) -> Result<Vec<FileProps>> {
	let dir = fs::read_dir(folder)?;
	let files = dir
		.filter_map(|dir_entry| {
			let path = dir_entry.ok()?.path();

			let id = path.file_stem()?.to_str()?.to_string();
			let extension = path.extension()?.to_str()?.to_string();

			if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) { return None; }

			Some(FileProps { id, extension })
		})
		.collect();

	Ok(files)
}
