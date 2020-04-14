use day01::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let masses = read_masses("input.txt")?;
	let base_fuel: i32 = masses.iter().map(|m| calc_fuel(*m)).sum();

	println!("Fuel necessary to lift all modules: {} liters.", base_fuel);

	println!(
		"Actual total fuel required, including fuel for the fuel: {} liters",
		base_fuel + calc_additional_fuel(base_fuel)
	);
	Ok(())
}
