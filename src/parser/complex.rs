//! Parser function for complex types (list, map)

use super::{Expr, Parser, Value};
use crate::{ErrorKind, Result};

impl Parser<'_> {
	pub fn parse_map(&mut self) -> Result<Expr> {
		todo!()
	}

	pub fn parse_list(&mut self) -> Result<Expr> {
		todo!()
	}
}
