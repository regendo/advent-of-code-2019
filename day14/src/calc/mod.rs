use std::collections::HashMap;

use crate::parse::Reaction;

#[derive(Debug, Clone)]
struct Resources<'a> {
	required: HashMap<&'a str, u8>,
	leftover: HashMap<&'a str, u8>,
}

pub fn what_creates<'a, 'b>(haystack: &'a [Reaction<'b>], needle: &str) -> Vec<&'a Reaction<'b>> {
	haystack.iter().filter(|r| r.output.0 == needle).collect()
}

fn react(reaction: &Reaction, amount: u8, resources: Resources) {}
