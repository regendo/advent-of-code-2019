use day02::{base_fuel_for_modules, calc_total_fuel};

fn main() {
	println!(
		"Actual total fuel required, including fuel for the fuel: {} liters",
		calc_total_fuel(base_fuel_for_modules())
	);
}
