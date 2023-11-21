#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod error;
mod parser;

pub use error::{Error, ErrorKind, Result};
pub use parser::*;

/// Parse a YADIL message.
pub fn parse(input: &[u8]) -> Result<parser::Message> {
	parser::Parser::new(input).parse()
}

/// Converts an index to a line and column.
pub fn index_to_line_col(input: &[u8], index: usize) -> (usize, usize) {
	let mut line = 1;
	let mut col = 2;

	for &byte in input.iter().take(index) {
		if byte == b'\n' {
			line += 1;
			col = 1;
		} else {
			col += 1;
		}
	}

	(line, col)
}
