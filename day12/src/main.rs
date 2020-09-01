use day12::{advance_time, total_energy, Moon, Pos};
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
	let mut moons = input()?.iter().map(Moon::new).collect::<Vec<Moon>>();
	for _ in 1..=1000 {
		advance_time(&mut moons)?;
	}
	println!(
		"After 1000 steps, there's a total energy of {} in the system.",
		total_energy(&moons)
	);

	Ok(())
}
