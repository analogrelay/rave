use term;
use term::Terminal;

use std::io::Writer;
use std::io::stdio::{stdout, stderr, StdWriter};

struct DiagnosticService {
	outterm: Box<Terminal<Box<Writer:Send>>>,
	errterm: Box<Terminal<Box<Writer:Send>>>,
	out: Box<Writer>,
	err: Box<Writer>
}

impl DiagnosticService {
	fn new() -> Box<DiagnosticService> {
		box DiagnosticService { 
			outterm: term::stdout().unwrap(), 
			errterm: term::stderr().unwrap(),
			out: box stdout(),
			err: box stderr()
		}
	}

	pub fn err(&mut self, message: &str) {
		self.errterm.fg(term::color::RED).unwrap();
		self.err.write_str("error: ");
		self.errterm.fg(term::color::WHITE).unwrap();
		self.err.write_line(message);
	}
}

pub fn init() -> Box<DiagnosticService> {
	DiagnosticService::new()
}