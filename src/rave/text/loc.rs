use std::fmt::{Formatter, Show, FormatError};
use std::string::String;

pub struct SourceLocation {
	offset: int,
	file: String,
	line: int,
	column: int
}

impl SourceLocation {
	fn new(file: &str, offset: int, line: int, column: int) -> SourceLocation {
		assert!(offset > 0);
		assert!(line > 0);
		assert!(column > 0);

		SourceLocation { 
			offset: offset, 
			file: String::from_str(file),
			line: line, 
			column: column 
		}
	}
}

impl Show for SourceLocation {
	fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
		write!(f, "{}:{}:{}", self.file, self.line + 1, self.column + 1)
	}
}

#[cfg(test)]
mod test {
	use super::SourceLocation;

	#[test]
	#[should_fail]
	pub fn test_negative_offset() {
		SourceLocation::new("foo", -1, 0, 0);
	}

	#[test]
	#[should_fail]
	pub fn test_negative_line() {
		SourceLocation::new("foo", 0, -1, 0);
	}

	#[test]
	#[should_fail]
	pub fn test_negative_column() {
		SourceLocation::new("foo", 0, 0, -1);
	}

	#[test]
	pub fn test_show_impl() {
		let loc = SourceLocation::new("file.js", 48, 1, 41);
		assert_eq!("file.js:2:42", format!("{}", loc).as_slice());
	}
}