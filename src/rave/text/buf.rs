use std::io::Reader;
use std::string::String;

use super::SourceLocation;

pub struct PositionTaggedCharacter {
	chr: char,
	location: SourceLocation
};

pub struct TextBuffer {
	putback_buffer: String,
	reader: Box<Reader>
	location: SourceLocation
};

impl TextBuffer {
	pub fn new(r: Box<Reader>, fileName: str) -> TextBuffer {
		TextBuffer {
			putback_buffer: String::new(),
			reader: r,
			location: SourceLocation::new(fileName, 0, 0, 0)
		}
	}

	pub fn next_char(&mut self) -> PositionTaggedCharacter {

	}

	pub fn putback(&mut self, chr: char) {

	}

	pub fn current_location(&self) -> SourceLocation {
		self.location
	}
}