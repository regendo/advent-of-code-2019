fn calc_fuel(mass: i32) -> i32 {
	mass / 3 - 2
}

fn main() {
	// TODO: read inputs, then calc and sum
	println!("The total amount of fuel required for my spacecraft is: ");
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
		assert_eq!(33583, calc_fuel(100756));
	}
}
