use day14::{calc, parse};
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
	let raw_input = fs::read_to_string("input.txt")?;
	let reactions = parse::parse_reactions(&raw_input)?;
	println!("{:?}", calc::what_creates(&reactions, "FUEL"));

	Ok(())
}
