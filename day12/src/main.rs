use day12::Pos;
use std::convert::TryFrom;
use std::error::Error;
use std::fs::read_to_string;

fn input() -> Result<Vec<Pos>, Box<dyn Error>> {
	read_to_string("input.bib")?
		.trim()
		.lines()
		.map(Pos::try_from)
		.collect()
}

fn main() -> Result<(), Box<dyn Error>> {
	let positions = input()?;
	for pos in positions {
		println!("Position {:?}", pos);
	}

	Ok(())
}
