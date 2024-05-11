use std::time::{SystemTime, UNIX_EPOCH};

pub fn timestamp() -> String {
	SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string()
}