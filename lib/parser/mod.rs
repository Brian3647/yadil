//! Parser of the YADIL specification, in rust.

mod complex;
mod literals;

use std::collections::HashMap;

use crate::{Error, ErrorKind, Result};

/// Any valid value.
#[derive(Debug, Clone)]
pub enum Value {
	String(String),
	Unsigned(usize),
	Signed(isize),
	Float(f64),
	Bool(bool),
	List(Vec<Value>),
	Map(HashMap<Vec<u8>, Value>),
}

#[derive(Debug, Clone)]
pub struct TypedValue {
	pub value: Value,
	pub data_type: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Assign(pub Vec<u8>, pub Value);

#[derive(Debug, Clone)]
pub struct Message(pub HashMap<Vec<u8>, Value>);

pub struct Parser<'a> {
	pub input: &'a [u8],
	pub index: usize,
}

impl<'src> Parser<'src> {
	/// The start bytes of a data type.
	///
	/// Contains the following: s (string or sint), u (unsigned), i (signed),
	/// f (float), b (byte), l (list), m (map)
	pub const DATA_TYPE_START_BYTES: [u8; 7] = [b's', b'u', b'i', b'f', b'b', b'l', b'm'];

	/// Bytes to ignore.
	pub const IGNORE_BYTES: [u8; 4] = [b' ', b'\n', b'\r', b'\t'];

	pub const ASCII_NINE: u8 = b'9';
	pub const ASCII_ZERO: u8 = b'0';

	pub fn new(input: &'src [u8]) -> Parser {
		Parser { input, index: 0 }
	}

	pub fn parse(&mut self) -> Result<Message> {
		let len = self.input.len();

		if self.input.is_empty() {
			return Ok(Message(HashMap::new()));
		}

		let mut body = HashMap::new();

		while self.index < len {
			// Avoiding "Cannot borrow `self.input` as mutable more than once at a time"
			let byte = self.input[self.index];
			match byte {
				0 => break,                        // End of message (null byte)
				b' ' | b'\n' | b'\r' | b'\t' => {} // Initial whitespaces & newlines are ignored,
				b'#' => {
					// Comments
					while self.index < len {
						self.index += 1;

						if self.maybe_escaped(self.input[self.index], b'#') {
							break;
						}
					}
				}
				other if Self::DATA_TYPE_START_BYTES.contains(&other) => {
					let Assign(key, value) = self.parse_assign_start()?;
					body.insert(key, value);
				}
				other => {
					return Err(self.error(
						ErrorKind::UnexpectedChar,
						format!("Expected expression, got `{}`", other as char),
					))
				}
			}

			self.index += 1;
		}

		Ok(Message(body))
	}

	fn parse_assign_start(&mut self) -> Result<Assign> {
		let mut data_type = vec![];

		while let Some(next) = self.next() {
			if self.maybe_escaped(next, b'@') {
				break;
			} else if Self::IGNORE_BYTES.contains(&next) {
				continue;
			}

			data_type.push(next);
		}

		match &data_type[..] {
			b"s" | b"str" => self.string_assign(),
			b"u" | b"uint" => self.unsigned_assign(),
			b"i" | b"sint" => self.signed_assign(),
			b"f" | b"float" => self.float_assign(),
			b"b" | b"bool" => self.bool_assign(),
			b"l" | b"list" => todo!(), // self.parse_list_assign(),
			b"m" | b"map" => todo!(),  // self.parse_map(),
			_ => Err(self.error(ErrorKind::UnexpectedChar, "Invalid data type")),
		}
	}

	fn error(&self, kind: ErrorKind, message: impl Into<String>) -> Error {
		Error::new(kind, message.into(), self.index)
	}

	fn to_utf8(&self, input: Vec<u8>) -> Result<String> {
		String::from_utf8(input).map_err(|_| self.error(ErrorKind::WrongValue, "Invalid utf8"))
	}

	/// Returns `true` if the current byte is the expected byte and the previous byte is not a
	/// backslash (escape symbol).
	fn maybe_escaped(&self, current: u8, expected: u8) -> bool {
		current == expected && self.index > 0 && self.input[self.index - 1] != b'\\'
	}
}

impl Iterator for Parser<'_> {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		if self.index >= self.input.len() {
			return None;
		}

		self.index += 1;
		Some(self.input[self.index - 1])
	}
}
