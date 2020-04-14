// TODO: re-use day01 as a module instead of using this cached value
fn base_fuel_for_modules() -> i32 {
	3_386_686
}

// TODO: import function from day01 instead
fn calc_fuel(mass: i32) -> i32 {
	mass / 3 - 2
}

/// Total amount of fuel necessary to lift the spaceship. This includes fuel to lift the fuel.
///
/// ## Spec
///
/// ```
/// assert_eq!(calc_total_fuel(14), 2);
/// assert_eq!(calc_total_fuel(1969), 966);
/// assert_eq!(calc_total_fuel(100_756), 50_346);
/// ```
///
fn calc_total_fuel(base_fuel: i32) -> i32 {
	let mut total_fuel = 0;
	let mut additional_fuel = base_fuel;

	while additional_fuel > 0 {
		total_fuel += additional_fuel;
		additional_fuel = calc_fuel(additional_fuel);
	}

	total_fuel
}

fn main() {
	println!(
		"Actual total fuel required, including fuel for the fuel: {} liters",
		calc_total_fuel(base_fuel_for_modules())
	);
}
