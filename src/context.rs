use crate::{file::File, util::*, vault::Vault};

pub struct Context<'c> {
	pub vault: Vault,
	pub file: Result<&'c mut File>,
}
