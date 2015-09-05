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

pub fn welcome() {
	println!("\n{}\n\n{}\n", make_title("MCMUCH Website Server"),
	                                   "Idea from RiskyKen, VicNightfall, and skyem123.\n\
									    Experimental server by skyem123.\n\
									    API by RX14 and StrikingWolf, with encouragement from trajing.\n\
									    This webserver by skyem123.\n");
}
