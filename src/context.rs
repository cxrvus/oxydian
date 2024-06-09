use crate::{file::File, util::*, vault::Vault};

pub struct Context {
	pub vault: Vault,
	pub file: Result<File>,
}
