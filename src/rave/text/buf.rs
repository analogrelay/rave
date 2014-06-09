use std::io::{Reader, IoResult};
use std::str::utf8_char_width;
use std::string::String;
use std::vec::Vec;

use loc::{Loc, LocTracker};
use err::SourceError;

pub struct TextBuffer<B> {
	putback_buffer: String,
	buffer: Box<B>,
	tracker: LocTracker
}

impl<B:Buffer> TextBuffer<B> {
	pub fn new(b: Box<B>, fileName: &str) -> TextBuffer<B> {
		TextBuffer {
			putback_buffer: String::new(),
			buffer: b,
			tracker: LocTracker::new(fileName)
		}
	}

	pub fn next(&mut self) -> IoResult<char> {
		// Read a char from the putback buffer if present, otherwise from the source buffer
		let cur_char = if self.putback_buffer.len() > 0 {
			match self.putback_buffer.pop_char() {
				Some(v) => v,
				None => unreachable!() // We just ensured there was something to pop!
			}
		} else {
			match self.buffer.read_char() {
				Ok(chr) => chr,
				e @ Err(_) => return e
			}
		};

		// Advance the location tracker
		self.tracker.advance(cur_char);

		// Return current character
		Ok(cur_char)
	}

	pub fn putback(&mut self, chr: char) {

	}

	pub fn current_location(&self) -> Loc {
		self.tracker.current()
	}
}