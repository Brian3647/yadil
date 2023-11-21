use std::{collections::HashMap, fs};

use yadil::{parse, Value};

fn main() {
	let path = std::env::args()
		.nth(1)
		.expect("No path provided (usage: yadil <path>)");

	let bytes = match fs::read(&path) {
		Ok(bytes) => bytes,
		Err(err) => {
			eprintln!("Error reading file: {err}");
			return;
		}
	};

	let message = match parse(&bytes) {
		Ok(message) => message,
		Err(err) => {
			let (line, col) = yadil::index_to_line_col(&bytes, err.index);
			eprintln!("Error parsing file at ({path}:{line}:{col}): {err:#?}");
			return;
		}
	};

	let utf8_map: HashMap<String, &Value> = message
		.0
		.iter()
		.map(|(key, value)| (String::from_utf8_lossy(key).into_owned(), value))
		.collect();

	println!("{:#?}", utf8_map);
}
