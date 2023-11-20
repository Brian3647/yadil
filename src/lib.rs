#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod error;
pub mod parser;

pub use error::{Error, ErrorKind, Result};
