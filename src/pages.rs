use handlebars_iron::{Template, HandlebarsEngine};

use serde_json::builder::ObjectBuilder;

use serde_json::value::Value;

use iron::prelude::*;
use iron::status;

use helpers::value_to_json;

fn make_simple_content(title: String, content: String) -> Value {
	let object = ObjectBuilder::new()
		.insert("title".to_string(), title)
		.insert("content".to_string(), content);
	return object.unwrap();
}

pub fn pages() -> Chain {
	let mut chain = Chain::new(|_: &mut Request| {
		let mut resp = Response::new();

		resp.set_mut(Template::new("simple_content", value_to_json(make_simple_content("MCMUCH".to_string(), "This is a test".to_string())))).set_mut(status::Ok);
    	Ok(resp)
    });
	chain.link_after(HandlebarsEngine::new("templates/", ".hbs"));
	return chain;
}
