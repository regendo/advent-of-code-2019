use day14::{calc, parse};
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
	let raw_input = fs::read_to_string("input.txt")?;
	let reactions = parse::parse_reactions(&raw_input)?;
	let resources = calc::create_next_chemical(&reactions, Default::default())?;

	println!(
		"Used {} ORE to create 1 FUEL.",
		resources.required.get("ORE").unwrap()
	);

	Ok(())
}
