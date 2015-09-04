extern crate term;

use std::io;
use std::io::BufReader;
use std::io::BufRead;

fn make_rule(with: &str, len: usize) -> String {
	let mut line = String::with_capacity(len);
	for _ in 0..len {
		line = line + with;
	}
	return line;
}

fn make_title(text: &str) -> String {
	let rule = make_rule("=", text.len());
	return text.to_string() + "\n" + &rule;
}

fn read_line() -> String {
	let mut reader = BufReader::new(io::stdin());
	let mut line = String::new();
	reader.read_line(&mut line).unwrap();
	return line
}

pub fn welcome() {
	println!("\n{}\n", make_title("MCMUCH Website Server"));
}

pub fn run() {
	let mut t = term::stdout().unwrap();

	loop {
		write!(t, "> ").unwrap();
		t.flush().unwrap();
		println!("{}", read_line())
	}
}
