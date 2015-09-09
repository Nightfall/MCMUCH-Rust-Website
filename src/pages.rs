use handlebars_iron::{Template, HandlebarsEngine};

use serde_json::builder::ObjectBuilder;
use serde_json::value::Value;
use serde_json::de;

use iron::prelude::*;
use iron::status;
use router::Router;

use std::fs::File;

use helpers::value_to_json;

pub fn make_simple_content(title: String, content: String) -> Value {
	return ObjectBuilder::new()
		.insert("title".to_string(), title)
		.insert("content".to_string(), content)
		.unwrap();
}

pub fn handle() -> Chain {
	// Make the layout
	let mut router = Router::new();

	router.get("/", |_: &mut Request| {
		let mut resp = Response::new();
		resp.set_mut(Template::new("simple_content", value_to_json(de::from_reader(File::open("content/index.json").unwrap()).unwrap()))).set_mut(status::Ok);
    	return Ok(resp);
	});
	router.get("/about/", |_: &mut Request| {
		let mut resp = Response::new();
		resp.set_mut(Template::new("simple_content", value_to_json(de::from_reader(File::open("content/about.json").unwrap()).unwrap()))).set_mut(status::Ok);
		return Ok(resp);
	});

	let mut chain = Chain::new(router);
	chain.link_after(HandlebarsEngine::new("templates/", ".hbs"));
	return chain;
}
