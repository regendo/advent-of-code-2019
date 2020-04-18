use day06::{build_solar_system, count_total_orbits, read_input};
use std::rc::Rc;

fn main() {
	let pairs = read_input("input.txt");
	let solar_system = build_solar_system(pairs.as_slice());
	let orbits = count_total_orbits(&solar_system);

	println!("This map data contains a total of {} orbits.", orbits);

	if let (Some(you), Some(san)) = (solar_system.get("YOU"), solar_system.get("SAN")) {
		if let Some(distance) = (*you).transfers_between_orbits(Rc::clone(san)) {
			println!("{} orbital transfers required to reach Santa!", distance);
		}
	}
}
