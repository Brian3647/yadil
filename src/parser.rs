//! Parser of the YADIL specification, in rust.

use std::collections::HashMap;

use crate::{Error, ErrorKind, Result};

/// Any valid value.
#[derive(Debug, Clone)]
pub enum Value {
	String(String),
	Unsigned(usize),
	Signed(isize),
	Byte(u8),
	Float(f64),
	Bool(bool),
	Empty,
	List(Vec<Expr>),
	Map(HashMap<Value, Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr {
	Assign(Vec<u8>, Value),
	Empty,
}

#[derive(Debug, Clone)]
pub struct Message {
	pub body: Vec<Expr>,
}

pub struct Parser<'a> {
	pub input: &'a [u8],
	pub index: usize,
}

impl<'src> Parser<'src> {
	/// The start bytes of a data type.
	///
	/// Contains the following: s (string or sint), u (unsigned), i (signed),
	/// f (float), b (byte), l (list), m (map), x (byte list)
	pub const DATA_TYPE_START_BYTES: [u8; 8] = [b's', b'u', b'i', b'f', b'b', b'l', b'm', b'x'];

	pub const ASCII_NINE: u8 = b'9';
	pub const ASCII_ZERO: u8 = b'0';

	pub fn new(input: &'src [u8]) -> Parser {
		Parser { input, index: 0 }
	}

	pub fn parse(&mut self) -> Result<Message> {
		let len = self.input.len();

		if self.next().is_none() {
			return Ok(Message {
				body: vec![Expr::Empty],
			});
		}

		let mut body = vec![];
		let mut index = self.index;

		while index < len {
			let byte = self.input[index];
			match byte {
				0 => break,                        // End of message (null byte)
				b' ' | b'\n' | b'\r' | b'\t' => {} // Initial whitespaces & newlines are ignored
				b'[' => body.push(self.parse_list()?),
				other if Self::DATA_TYPE_START_BYTES.contains(&other) => {
					body.push(self.parse_expr(other)?)
				}
				_ => return Err(self.error(ErrorKind::UnexpectedChar, "Expected expression")),
			}
			index += 1;
		}

		Ok(Message { body })
	}

	fn parse_expr(&mut self, start: u8) -> Result<Expr> {
		let mut data_type = vec![start];

		for next in self.by_ref() {
			if next == b'@' {
				break;
			}

			data_type.push(next);
		}

		match &data_type[..] {
			b"s" | b"str" => self.parse_string(),
			b"u" | b"uint" => self.parse_unsigned(),
			b"i" | b"sint" => self.parse_signed(),
			b"f" | b"float" => self.parse_float(),
			b"b" | b"bool" => self.parse_bool(),
			b"l" | b"list" => self.parse_list(),
			b"m" | b"map" => self.parse_map(),
			_ => Err(self.error(ErrorKind::UnexpectedChar, "Invalid data type")),
		}
	}

	fn parse_string(&mut self) -> Result<Expr> {
		let (ident, input) = self.parse_custom_type()?;
		let input = self.to_utf8(input)?;
		Ok(Expr::Assign(ident, Value::String(input)))
	}

	fn parse_unsigned(&mut self) -> Result<Expr> {
		let (ident, input) = self.parse_custom_type()?;
		let mut total: usize = 0;

		for byte in input.iter() {
			if !(Self::ASCII_ZERO..=Self::ASCII_NINE).contains(byte) {
				return Err(self.error(ErrorKind::WrongValue, "Invalid unsigned value"));
			}

			total += (byte - Self::ASCII_ZERO) as usize;
		}

		Ok(Expr::Assign(ident, Value::Unsigned(total)))
	}

	fn parse_signed(&mut self) -> Result<Expr> {
		todo!()
	}

	fn parse_float(&mut self) -> Result<Expr> {
		todo!()
	}

	fn parse_bool(&mut self) -> Result<Expr> {
		todo!()
	}

	fn parse_map(&mut self) -> Result<Expr> {
		todo!()
	}

	fn parse_list(&mut self) -> Result<Expr> {
		todo!()
	}

	fn parse_custom_type(&mut self) -> Result<(Vec<u8>, Vec<u8>)> {
		let mut data = vec![];
		let mut ident = vec![];
		let mut in_value = false;

		while let Some(next) = self.next() {
			if next == b'=' {
				if ident.is_empty() {
					return Err(self.error(ErrorKind::EmptyIdent, "Identifier is empty"));
				}

				in_value = true;
				continue;
			} else if next == b';' {
				if ident.is_empty() {
					return Err(self.error(
						ErrorKind::UnexpectedChar,
						"Unexpected semicolon before expr start",
					));
				} else if !in_value || data.is_empty() {
					return Err(self.error(ErrorKind::WrongValue, "Expected value in expr"));
				}

				break;
			}

			if in_value {
				data.push(next);
			} else {
				ident.push(next);
			}

			data.push(next);
		}

		Ok((ident, data))
	}

	fn error(&self, kind: ErrorKind, message: &'src str) -> Error {
		Error::new(kind, format!("error at index {}: {}", self.index, message))
	}

	fn to_utf8(&self, input: Vec<u8>) -> Result<String> {
		String::from_utf8(input).map_err(|_| self.error(ErrorKind::WrongValue, "Invalid utf8"))
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
