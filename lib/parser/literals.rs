//! Parser function for literal types (string, unsigned, signed, float, bool)

use super::{Assign, Parser, Value};
use crate::{ErrorKind, Result};

macro_rules! create_assign_parser {
	($name:ident, $ty:ident, $parser:ident) => {
		#[inline]
		#[doc(hidden)]
		pub fn $name(&mut self) -> Result<Assign> {
			let (ident, input) = self.parse_assign()?;
			Ok(Assign(ident, self.$parser(input)?))
		}
	};
}

macro_rules! create_assign_parsers {
	($($name:ident, $ty:ident, $parser:ident);* $(;)?) => {
		$(create_assign_parser!($name, $ty, $parser);)*
	};
}

impl Parser<'_> {
	#[inline]
	pub fn parse_string(&mut self, bytes: Vec<u8>) -> Result<Value> {
		Ok(Value::String(self.to_utf8(bytes)?))
	}

	pub fn parse_unsigned(&mut self, bytes: Vec<u8>) -> Result<Value> {
		let mut total: usize = 0;

		for byte in bytes.iter() {
			if !(Self::ASCII_ZERO..=Self::ASCII_NINE).contains(byte) {
				return Err(self.error(
					ErrorKind::WrongValue,
					format!("Invalid unsigned value `{}`", *byte as char),
				));
			}

			total += (byte - Self::ASCII_ZERO) as usize;
		}

		Ok(Value::Unsigned(total))
	}

	pub fn parse_signed(&mut self, bytes: Vec<u8>) -> Result<Value> {
		let mut total: isize = 0;
		let mut is_negative = false;
		let mut in_number = false;

		for &byte in bytes.iter() {
			if byte == b'-' {
				if in_number {
					return Err(self.error(
						ErrorKind::WrongValue,
						"Found `-` after number rather than before",
					));
				}

				is_negative = !is_negative;
				in_number = true;
				continue;
			}

			if !(Self::ASCII_ZERO..=Self::ASCII_NINE).contains(&byte) {
				return Err(self.error(ErrorKind::WrongValue, "Invalid signed value"));
			}

			total += (byte - Self::ASCII_ZERO) as isize;
		}

		if is_negative {
			total = -total;
		}

		Ok(Value::Signed(total))
	}

	pub fn parse_float(&mut self, bytes: Vec<u8>) -> Result<Value> {
		let mut total = 0.0;
		let mut dec_count = 0;
		let mut is_negative = false;
		let mut in_number = false;
		let mut in_dec = false;

		for &byte in bytes.iter() {
			if byte == b'-' {
				if in_number {
					return Err(self.error(
						ErrorKind::WrongValue,
						"Found `-` after number rather than before",
					));
				}

				is_negative = !is_negative;
				in_number = true;
				continue;
			} else if byte == b'.' {
				if in_dec {
					return Err(self.error(
						ErrorKind::WrongValue,
						"Found `.` after decimal rather than before",
					));
				}

				in_dec = true;
				continue;
			}

			if !(Self::ASCII_ZERO..=Self::ASCII_NINE).contains(&byte) {
				return Err(self.error(ErrorKind::WrongValue, "Invalid float value"));
			}

			if in_dec {
				dec_count += 1;
				total += (byte - Self::ASCII_ZERO) as f64 / 10.0_f64.powi(dec_count);
			} else {
				total += (byte - Self::ASCII_ZERO) as f64;
			}
		}

		if is_negative {
			total = -total;
		}

		Ok(Value::Float(total))
	}

	#[inline]
	pub fn parse_bool(&mut self, bytes: Vec<u8>) -> Result<Value> {
		Ok(Value::Bool(match &bytes[..] {
			b"true" | b"t" => true,
			b"false" | b"f" => false,
			_ => return Err(self.error(ErrorKind::WrongValue, "Invalid bool value")),
		}))
	}

	pub fn parse_assign(&mut self) -> Result<(Vec<u8>, Vec<u8>)> {
		let mut data = vec![];
		let mut ident = vec![];
		let mut in_value = false;

		while let Some(next) = self.next() {
			if self.maybe_escaped(next, b'=') {
				if ident.is_empty() {
					return Err(self.error(ErrorKind::EmptyIdent, "Identifier is empty"));
				}

				in_value = true;
				continue;
			} else if self.maybe_escaped(next, b';') {
				if ident.is_empty() {
					return Err(self.error(
						ErrorKind::UnexpectedChar,
						"Unexpected semicolon before expr start",
					));
				} else if !in_value || data.is_empty() {
					return Err(self.error(ErrorKind::WrongValue, "Expected value in expr"));
				}

				break;
			} else if Self::IGNORE_BYTES.contains(&next) && (!in_value || data.is_empty()) {
				continue;
			}

			if in_value {
				data.push(next);
			} else {
				ident.push(next);
			}
		}

		// I'm not quite sure where, but somewhere index is growing when it shouldn't.
		// This fixes it.
		self.index -= 1;

		Ok((ident, data))
	}

	create_assign_parsers!(
		string_assign, String, parse_string;
		unsigned_assign, Unsigned, parse_unsigned;
		signed_assign, Signed, parse_signed;
		float_assign, Float, parse_float;
		bool_assign, Bool, parse_bool;
	);
}
