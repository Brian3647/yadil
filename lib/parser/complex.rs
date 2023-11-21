//! Parser function for complex types (list, map)

use super::{Assign, Parser, Value};
use crate::Result;

impl Parser<'_> {
	pub fn parse_map(&mut self) -> Result<Assign> {
		todo!()
	}

	pub fn parse_list(&mut self, _ty: Vec<u8>) -> Result<Value> {
		todo!()
	}

	pub fn parse_list_assign(&mut self) -> Result<Assign> {
		todo!()
	}

	fn _parse_list_value(&mut self, _ty: Vec<u8>) -> Result<Value> {
		todo!()
	}
}
