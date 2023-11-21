//! Parser function for literal types (string, unsigned, signed, float, bool)

use super::{Expr, Parser, Value};
use crate::{ErrorKind, Result};

macro_rules! create_assign_parser {
	($name:ident, $ty:ident, $parser:ident) => {
		#[inline]
		#[doc(hidden)]
		pub fn $name(&mut self) -> Result<Expr> {
			let (ident, input) = self.parse_assign()?;
			Ok(Expr::Assign(ident, Value::$ty(self.$parser(input)?)))
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
	fn parse_string(&mut self, bytes: Vec<u8>) -> Result<String> {
		self.to_utf8(bytes)
	}

	fn parse_unsigned(&mut self, bytes: Vec<u8>) -> Result<usize> {
		let mut total: usize = 0;

		for byte in bytes.iter() {
			if !(Self::ASCII_ZERO..=Self::ASCII_NINE).contains(byte) {
				return Err(self.error(ErrorKind::WrongValue, "Invalid unsigned value"));
			}

			total += (byte - Self::ASCII_ZERO) as usize;
		}

		Ok(total)
	}

	fn parse_signed(&mut self, bytes: Vec<u8>) -> Result<isize> {
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

		Ok(total)
	}

	fn parse_float(&mut self, bytes: Vec<u8>) -> Result<f64> {
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

		Ok(total)
	}

	#[inline]
	fn parse_bool(&mut self, bytes: Vec<u8>) -> Result<bool> {
		Ok(match &bytes[..] {
			b"true" | b"t" => true,
			b"false" | b"f" => false,
			_ => return Err(self.error(ErrorKind::WrongValue, "Invalid bool value")),
		})
	}

	fn parse_assign(&mut self) -> Result<(Vec<u8>, Vec<u8>)> {
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

	create_assign_parsers!(
		string_assign, String, parse_string;
		unsigned_assign, Unsigned, parse_unsigned;
		signed_assign, Signed, parse_signed;
		float_assign, Float, parse_float;
		bool_assign, Bool, parse_bool;
	);
}
