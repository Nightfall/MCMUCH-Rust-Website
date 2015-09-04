extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate term;

use std::path::Path;
use std::thread;

use iron::prelude::*;

use mount::Mount;
use staticfile::Static;

mod console;

fn main() {
	// Print a welcome message
	console::welcome();

	let mut mount = Mount::new();
	mount.mount("/", Static::new(Path::new("pages/")));

	thread::spawn(move || {
		Iron::new(mount).http("localhost:3000").unwrap();
	});

	console::run();
}
