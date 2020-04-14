use std::error::Error;
use std::fs;
use std::io;

fn calc_fuel(mass: i32) -> i32 {
	mass / 3 - 2
}

fn read_masses(file: &str) -> io::Result<Vec<i32>> {
	let input = fs::read_to_string(file)?;
	let mapped: Vec<i32> = input
		.split_whitespace()
		.map(|s| s.parse::<i32>().unwrap())
		.collect();
	Ok(mapped)
}

fn main() -> Result<(), Box<dyn Error>> {
	let masses = read_masses("input.txt")?;
	let fuel: i32 = masses.iter().map(|m| calc_fuel(*m)).sum();

	println!(
		"The total amount of fuel required for my spacecraft is: {} liters.",
		fuel
	);
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn spec1() {
		assert_eq!(2, calc_fuel(12));
	}
	#[test]
	fn spec2() {
		assert_eq!(2, calc_fuel(14));
	}
	#[test]
	fn spec3() {
		assert_eq!(654, calc_fuel(1969));
	}
	#[test]
	fn spec4() {
		assert_eq!(33_583, calc_fuel(100_756));
	}
}
