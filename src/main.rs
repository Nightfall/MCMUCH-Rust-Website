extern crate iron;
extern crate mount;
extern crate staticfile;

use std::path::Path;

use iron::prelude::*;

use mount::Mount;
use staticfile::Static;

fn main() {
	let mut mount = Mount::new();

	mount.mount("/", Static::new(Path::new("pages/")));

	Iron::new(mount).http("localhost:3000").unwrap();
}
