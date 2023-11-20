//! Parser of the YADIL specification, in rust.

use std::collections::HashMap;

use crate::{Error, ErrorKind, Result};

/// The start byte for a YADIL message (0x59, 0x2E).
pub const START: &[u8; 2] = b"Y.";

/// The latest version of the YADIL specification.
pub const VERSION: u8 = 1;

const ASCII_ZERO: u8 = 48;
const ASCII_NINE: u8 = 58;

/// Any valid value.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Value<'a> {
	String(&'a str),
	Unsigned(usize),
	Signed(isize),
	Byte(u8),
	Float(f64),
	Bool(bool),
	Empty,
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
	Value(Value<'a>),
	List(Vec<Expr<'a>>),
	Map(HashMap<Value<'a>, Expr<'a>>),
}

#[derive(Debug, Clone)]
pub struct Message<'a> {
	pub version: u8,
	pub body: Expr<'a>,
}

pub struct Parser<'a> {
	pub input: &'a [u8],
	pub index: usize,
}

impl<'a> Parser<'a> {
	pub fn new(input: &[u8]) -> Parser {
		Parser { input, index: 0 }
	}

	pub fn parse(&mut self) -> Result<Message<'a>> {
		let (version, index) = Self::parse_version(self.input)?;
		self.index = index as usize;

		if self.next().is_none() {
			return Ok(Message {
				version,
				body: Expr::Value(Value::Empty),
			});
		}

		for byte in self {
			match byte {
				0 => break,                        // End of message (null byte)
				b' ' | b'\n' | b'\r' | b'\t' => {} // Whitespaces & newlines are ignored
				b';' => return Err(Error::new(ErrorKind::ParseError, "Unexpected semicolon")),
				_ => todo!(), // TODO:
			}
		}

		let body = Expr::List(vec![Expr::Value(Value::Empty)]);
		Ok(Message { version, body })
	}

	/// Parses the first 4 bytes of the message, which should be the
	/// start bytes, the version and a semicolon if there's more data.
	/// Using `&self` here fails in the borrow checker.
	fn parse_version(input: &[u8]) -> Result<(u8, u8)> {
		if input.len() <= 2 {
			return Err(Error::new(
				ErrorKind::MessageTooShort,
				"Message is less than 3 bytes long",
			));
		} else if input[0..2] != *START {
			return Err(Error::version_not_supported());
		}

		let mut version = input[3];

		// The version apparently is a character converted to a byte (x + 48, if x ∈ [0, 9])
		if version > ASCII_ZERO && version < ASCII_NINE {
			version -= ASCII_ZERO;
		} else {
			return Err(Error::version_not_supported());
		}

		let is_empty = input.len() == 3;

		if version > VERSION {
			return Err(Error::version_not_supported());
		} else if !is_empty && input[4] != b';' {
			return Err(Error::new(
				ErrorKind::ParseError,
				"Version is not followed by a semicolon & message is not empty",
			));
		}

		Ok((version, if is_empty { 3 } else { 4 }))
	}
}

impl Iterator for Parser<'_> {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		if self.index > self.input.len() {
			return None;
		}

		let byte = self.input[self.index];
		self.index += 1;
		Some(byte)
	}
}
