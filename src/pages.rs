use handlebars_iron::{Template, HandlebarsEngine};

use serde_json::builder::ObjectBuilder;

use serde_json::value::Value;

use iron::prelude::*;
use iron::status;
use router::Router;

use helpers::value_to_json;
use content;

pub fn make_simple_content(title: String, content: String) -> Value {
	let object = ObjectBuilder::new()
		.insert("title".to_string(), title)
		.insert("content".to_string(), content);
	return object.unwrap();
}

pub fn pages() -> Chain {
	// Make the layout
	let mut router = Router::new();

	router.get("/", |_: &mut Request| {
		let mut resp = Response::new();
		resp.set_mut(Template::new("simple_content", value_to_json(content::make_index()))).set_mut(status::Ok);
    	return Ok(resp);
	});

	let mut chain = Chain::new(router);
	chain.link_after(HandlebarsEngine::new("templates/", ".hbs"));
	return chain;
}
