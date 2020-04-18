use day06::{build_solar_system, count_total_orbits, read_input};

fn main() {
	let pairs = read_input("input.txt");
	let solar_system = build_solar_system(pairs.as_slice());
	let orbits = count_total_orbits(&solar_system);

	println!("This map data contains a total of {} orbits.", orbits);
}
