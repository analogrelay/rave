extern crate term;

use std::vec::Vec;

pub mod diag;
pub mod driver;

fn main() {
	// Get the name of the file to run
	let mut args = std::os::args();
	if args.len() < 2 {
		usage()
	} else {
		let prog = args.shift().unwrap();
		let file = args.shift().unwrap();
		driver::run(prog, file);
	}
}

fn usage() {
	println!("rave <file>");
}