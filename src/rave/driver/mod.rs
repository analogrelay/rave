use diag;

pub fn run(program: &str, source_file: &str) {
	let mut log = diag::init();
	let p = Path::new(source_file);
	if !p.exists() {
		log.err(format!("Input not found: {}", source_file));
	}
}