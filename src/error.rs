//! Error type for the library.

/// A result type, containing either a value or an error.
pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

/// An error, containing its kind and a message.
pub struct Error<'a> {
	/// The kind of error.
	pub kind: ErrorKind,
	/// Detailed information of the error.
	pub message: &'a str,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ErrorKind {
	UnexpectedChar,
	EmptyIdent,
	WrongValue,
}

impl<'a> Error<'a> {
	/// Create a new error.
	pub const fn new(kind: ErrorKind, message: &str) -> Error {
		Error { kind, message }
	}
}
