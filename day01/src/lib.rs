use std::cmp::max;
use std::fs;
use std::io;

/// Calculate the theoretical amount of fuel needed to lift a mass into orbit, ignoring the fuel's own mass.
///
/// ## Spec
/// ```
/// # use day01::calc_basic_fuel;
/// assert_eq!(calc_basic_fuel(12), 2);
/// assert_eq!(calc_basic_fuel(14), 2);
/// assert_eq!(calc_basic_fuel(1969), 654);
/// assert_eq!(calc_basic_fuel(100_756), 33_583);
/// assert_eq!(calc_basic_fuel(0), 0);
/// ```
fn calc_basic_fuel(mass: i32) -> i32 {
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

/// Calculate the total fuel required to lift both a mass and its fuel into orbit.
///
/// ## Spec
///
/// ```
/// # use day01::calc_total_fuel;
/// assert_eq!(calc_total_fuel(14), 2);
/// assert_eq!(calc_total_fuel(1969), 966);
/// assert_eq!(calc_total_fuel(100_756), 50_346);
/// ```
///
fn calc_total_fuel(mass: i32) -> i32 {
	let mut total_fuel = calc_basic_fuel(mass);
	let mut additional_fuel = calc_basic_fuel(total_fuel);

	while additional_fuel > 0 {
		total_fuel += additional_fuel;
		additional_fuel = calc_basic_fuel(additional_fuel);
	}

	total_fuel
}

/// Calculate the amount of fuel needed to lift all modules into orbit -- ignoring the fuel's own weight.
/// Consider using [total_spaceship_fuel](#total_spaceship_fuel) instead.
pub fn total_spaceship_fuel_naive(modules: &[i32]) -> i32 {
	modules.iter().map(|m| calc_basic_fuel(*m)).sum()
}

/// Calculate the amount of fuel needed to lift all modules into orbit, and to lift the fuel.
pub fn total_spaceship_fuel(modules: &[i32]) -> i32 {
	modules.iter().map(|m| calc_total_fuel(*m)).sum()
}
