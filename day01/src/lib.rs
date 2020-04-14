use std::cmp::max;
use std::fs;
use std::io;

/// Calculate the theoretical amount of fuel needed to lift a mass into orbit, ignoring the fuel's own mass.
///
/// ## Spec
/// ```
/// # use day01::calc_fuel;
/// assert_eq!(calc_fuel(12), 2);
/// assert_eq!(calc_fuel(14), 2);
/// assert_eq!(calc_fuel(1969), 654);
/// assert_eq!(calc_fuel(100_756), 33_583);
/// assert_eq!(calc_fuel(0), 0);
/// ```
pub fn calc_fuel(mass: i32) -> i32 {
	max(0, mass / 3 - 2)
}

/// Read the individual modules' masses from an input file.
pub fn read_masses(file: &str) -> io::Result<Vec<i32>> {
	let input = fs::read_to_string(file)?;
	let mapped: Vec<i32> = input
		.split_whitespace()
		.map(|s| s.parse::<i32>().unwrap())
		.collect();
	Ok(mapped)
}

/// Calculate the additional fuel needed to lift an amount of fuel. (And to lift that additional fuel, and so on.)
///
/// ## Spec
///
/// ```
/// # use day01::calc_additional_fuel;
/// assert_eq!(calc_additional_fuel(2), 0);
/// assert_eq!(calc_additional_fuel(654), 966 - 654);
/// assert_eq!(calc_additional_fuel(33_583), 50_346 - 33_583);
/// ```
///
pub fn calc_additional_fuel(base_fuel: i32) -> i32 {
	let mut total_fuel = 0;
	let mut additional_fuel = calc_fuel(base_fuel);

	while additional_fuel > 0 {
		total_fuel += additional_fuel;
		additional_fuel = calc_fuel(additional_fuel);
	}

	total_fuel
}
