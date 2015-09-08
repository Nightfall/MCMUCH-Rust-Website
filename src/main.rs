extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate logger;
extern crate handlebars_iron;
extern crate serde_json;
extern crate rustc_serialize;
extern crate router;

use std::path::Path;
use std::env;

use iron::prelude::*;
use logger::Logger;

use mount::Mount;
use staticfile::Static;

mod console;
mod pages;
mod helpers;

fn main() {
	// Print a welcome message
	console::welcome();

	// Get the address to listen on
	let address: &str = &env::args().nth(1).unwrap_or("localhost:3000".to_string());

	// Create the layout
	let mut mount = Mount::new();
	mount.mount("/styles", Static::new(Path::new("styles/")));
	mount.mount("/images", Static::new(Path::new("images/")));
	mount.mount("/favicon.ico", Static::new(Path::new("images/favicon.ico")));
	mount.mount("/", pages::handle());

	// Create the middleware chain for logging
	let mut chain = Chain::new(mount);
	chain.link(Logger::new(None));

	// Start the server
	let server = Iron::new(chain);
	println!("Starting server on {}...\n(press <ctrl>+'C' to stop server)\n", address);
	server.http(address).unwrap();
}
