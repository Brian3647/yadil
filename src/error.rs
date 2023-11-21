//! Error type for the library.

/// A result type, containing either a value or an error.
pub type Result<T> = std::result::Result<T, Error>;

/// An error, containing its kind and a message.
pub struct Error {
	/// The kind of error.
	pub kind: ErrorKind,
	/// Detailed information of the error.
	pub message: String,
	/// The index of the error in the input string.
	pub index: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ErrorKind {
	UnexpectedChar,
	EmptyIdent,
	WrongValue,
}

impl Error {
	/// Create a new error.
	pub const fn new(kind: ErrorKind, message: String, index: usize) -> Error {
		Error {
			kind,
			message,
			index,
		}
	}
}
