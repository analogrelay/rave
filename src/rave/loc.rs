use std::fmt::{Formatter, Show, FormatError};
use std::string::String;
use std::clone::Clone;

#[deriving(Clone)]
pub struct Loc {
	pub offset: int,
	pub file: String,
	pub line: int,
	pub column: int
}

impl Loc {
	fn new(file: &str, offset: int, line: int, column: int) -> Loc {
		assert!(offset >= 0);
		assert!(line >= 0);
		assert!(column >= 0);

		Loc { 
			offset: offset, 
			file: String::from_str(file),
			line: line, 
			column: column 
		}
	}
}

impl Show for Loc {
	fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
		write!(f, "{}:{}:{}", self.file, self.line + 1, self.column + 1)
	}
}

pub struct LocTracker {
	cur: Loc,
	prev: Option<char>
}

impl LocTracker {
	pub fn new(file: &str) -> LocTracker {
		LocTracker {
			cur: Loc::new(file, 0, 0, 0),
			prev: None
		}
	}

	// Gets a boolean indicating if the tracker is at BOF (Beginning of File)
	pub fn is_bof(&self) -> bool {
		match self.prev {
			Some(_) => false,
			None => true
		}
	}

	pub fn advance(&mut self, cur: char) {
		let prev = self.prev;
		self.prev = Some(cur);

		// 1. The character is a '\n' after a '\r'
		match (prev, cur) {
			// No previous character, initialize the buffer
			(None, x) => self.prev = Some(x),

			// First char of two-char newline: '\r\n'. We'll handle the line break later
			(Some('\r'), '\n') => self.incr_col(),
			// Single-char newline: '\r' or '\n', increment line
			(Some('\r'), _) | (Some('\n'), _) => self.incr_line(),
			// Other char, increment column
			_ => self.incr_col()
		}
	}

	pub fn current(&self) -> Loc {
		assert!(!self.is_bof());
		self.cur.clone()
	}

	fn incr_col(&mut self) {
		self.cur.offset += 1;
		self.cur.column += 1;
	}

	fn incr_line(&mut self) {
		self.cur.offset += 1;
		self.cur.line += 1;
		self.cur.column = 0;
	}
}

#[cfg(test)]
mod test {
	use super::{Loc, LocTracker};

	#[test]
	#[should_fail]
	pub fn test_negative_offset() {
		Loc::new("foo", -1, 0, 0);
	}

	#[test]
	#[should_fail]
	pub fn test_negative_line() {
		Loc::new("foo", 0, -1, 0);
	}

	#[test]
	#[should_fail]
	pub fn test_negative_column() {
		Loc::new("foo", 0, 0, -1);
	}

	#[test]
	pub fn test_zero_fields() {
		Loc::new("foo", 0, 0, 0);
	}

	#[test]
	pub fn test_pos_fields() {
		Loc::new("foo", 4, 2, 42);
	}

	#[test]
	pub fn test_show_impl() {
		let loc = Loc::new("file.js", 48, 1, 41);
		assert_eq!("file.js:2:42", format!("{}", loc).as_slice());
	}

	#[test]
	pub fn test_tracker_new() {
		let trk = new_trk();
		assert!(trk.is_bof());
	}

	#[test]
	#[should_fail]
	pub fn test_tracker_current_fails_when_bof() {
		let trk = new_trk();
		trk.current();
	}

	#[test]
	pub fn test_tracker_advance_non_newline() {
		let mut trk = new_trk();
		assert!(trk.is_bof());
		trk.advance('f');
		assert_trk(&trk, 0, 0, 0);
		trk.advance('o');
		assert_trk(&trk, 1, 0, 1);
		trk.advance('o');
		assert_trk(&trk, 2, 0, 2);
	}

	#[test]
	pub fn test_tracker_advance_single_char_newline() {
		let mut trk = new_trk();
		assert!(trk.is_bof());
		trk.advance('f');
		trk.advance('o');
		trk.advance('o');
		assert_trk(&trk, 2, 0, 2);
		trk.advance('\n'); // Newline itself is on line 0
		assert_trk(&trk, 3, 0, 3);
		trk.advance('b'); // First character after the newline is on line 1
		assert_trk(&trk, 4, 1, 0);
	}

	#[test]
	pub fn test_tracker_advance_single_char_carriage_return() {
		let mut trk = new_trk();
		assert!(trk.is_bof());
		trk.advance('f');
		trk.advance('o');
		trk.advance('o');
		assert_trk(&trk, 2, 0, 2);
		trk.advance('\r'); // Newline itself is on line 0
		assert_trk(&trk, 3, 0, 3);
		trk.advance('b'); // First character after the newline is on line 1
		assert_trk(&trk, 4, 1, 0);
	}

	#[test]
	pub fn test_tracker_advance_two_char_crlf() {
		let mut trk = new_trk();
		assert!(trk.is_bof());
		trk.advance('f');
		trk.advance('o');
		trk.advance('o');
		assert_trk(&trk, 2, 0, 2);
		trk.advance('\r'); // Newline itself is on line 0
		assert_trk(&trk, 3, 0, 3);
		trk.advance('\n');
		assert_trk(&trk, 4, 0, 4);
		trk.advance('b'); // First character after the newline is on line 1
		assert_trk(&trk, 5, 1, 0);
	}

	fn new_trk() -> LocTracker {
		LocTracker::new("testfile.js")
	}

	fn assert_trk(trk: &LocTracker, offset: int, line: int, column: int) {
		assert_eq!("testfile.js", trk.current().file.as_slice());
		assert_eq!(offset, trk.current().offset);
		assert_eq!(line, trk.current().line);
		assert_eq!(column, trk.current().column);
	}
}