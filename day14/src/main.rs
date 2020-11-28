use day14::parse;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
	let reactions = parse::parse_reactions(&fs::read_to_string("input.txt")?);

	Ok(())
}
