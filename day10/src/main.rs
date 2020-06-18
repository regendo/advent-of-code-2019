use day10::Point2D;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("input.bib")?;
	let map = Point2D::from_asteroid_map(input.split_whitespace());
	let station = Point2D::with_highest_visibility(&map).ok_or("No point found")?;
	println!(
		"Optimal location for a monitoring station is {:?}, with {} asteroids visible.",
		station,
		station.filter_visible(&map).len()
	);

	Ok(())
}
