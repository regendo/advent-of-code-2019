use day01::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let masses = read_masses("input.txt")?;
	let base_fuel = total_spaceship_fuel_naive(&masses);

	println!("Fuel necessary to lift all modules: {} liters.", base_fuel);

	println!(
		"Actual total fuel required, including fuel for the fuel: {} liters",
		total_spaceship_fuel(&masses)
	);
	Ok(())
}
