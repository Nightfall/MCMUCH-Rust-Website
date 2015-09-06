use handlebars_iron::{Template, HandlebarsEngine};

use iron::prelude::*;
use iron::status;

use rustc_serialize::json::Json;

pub fn pages() -> Chain {
	let mut chain = Chain::new(|_: &mut Request| {
		let mut resp = Response::new();

		resp.set_mut(Template::new("index", Json::from_str("{ \"title\": \"MCMUCH\"}").unwrap())).set_mut(status::Ok);
    	Ok(resp)
    });
	chain.link_after(HandlebarsEngine::new("templates/", ".hbs"));
	return chain;
}
