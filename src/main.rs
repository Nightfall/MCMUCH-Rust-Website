extern crate iron;
extern crate mount;
extern crate staticfile;

use std::path::Path;
use std::env;

use iron::prelude::*;

use mount::Mount;
use staticfile::Static;

mod console;

fn main() {
	// Print a welcome message
	console::welcome();

	// Get the address to listen on
	let address: &str = &env::args().nth(1).unwrap_or("localhost:3000".to_string());

	// Create the layout
	let mut mount = Mount::new();
	mount.mount("/", Static::new(Path::new("static/")));

	// Start the server
	println!("Starting server on {}...\n(press <ctrl>+'C' to stop server)", address);
	Iron::new(mount).http(address).unwrap();
}
