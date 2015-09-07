use serde_json::value::Value;

use pages::*;

pub fn make_index() -> Value {
	make_simple_content("MCMUCH Website".to_string(),
		r#"
			<p>Hello, you have reached the website for MCMUCH, which is currently being programmed right now, and so it is not up right now!<br />
			   However, If you want to see the source code, go to <a href="https://github.com/Nightfall">our GitHub organisation</a>, and here is a list on what we are currently working on:</p>
			<ul>
				<li><a href="https://github.com/Nightfall/MCMUCH-Ruby-API">MCMUCH-Ruby-API</a> is the server side code that we use for the API.</li>
				<li><a href="https://github.com/Nightfall/MCMUCH-Rust-Website">MCMUCH-Rust-Website</a> is the server side code for this website.</li>
			</ul>
		"#.to_string()
	)
}
