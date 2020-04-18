use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::hash::BuildHasher;
use std::rc::Rc;

#[derive(Debug)]
pub struct Satellite {
	name: String,
	orbit: RefCell<Option<Rc<Satellite>>>,
}

impl Satellite {
	fn new(name: &str) -> Rc<Satellite> {
		Rc::new(Satellite {
			name: name.to_string(),
			orbit: RefCell::new(None),
		})
	}

	fn count_orbits(&self) -> u32 {
		match &*self.orbit.borrow() {
			None => 0,
			Some(other) => 1 + other.count_orbits(),
}
	}
}

pub fn read_input(path: &str) -> Vec<(String, String)> {
	fs::read_to_string(path)
		.unwrap()
		.lines()
		.map(|l| {
			let mut split = l.split(')');
			(
				split.next().unwrap().to_string(),
				split.next().unwrap().to_string(),
			)
		})
		.collect()
}

pub fn build_solar_system(input: &[(&str, &str)]) -> HashMap<String, Rc<Satellite>> {
	let mut solar_system = HashMap::<String, Rc<Satellite>>::new();

	for (orb, sat) in input {
		solar_system
			.entry((*orb).to_string())
			.or_insert_with(|| Satellite::new(orb));
		solar_system
			.entry((*sat).to_string())
			.or_insert_with(|| Satellite::new(sat));

		// we need to get it here instead of above through `or_insert_with` because that would mutably borrow
		let satellite = solar_system.get(*sat).unwrap();
		*satellite.orbit.borrow_mut() = Some(Rc::clone(solar_system.get(*orb).unwrap()));
	}

	solar_system
}

pub fn count_total_orbits<S: BuildHasher>(solar_system: &HashMap<String, Rc<Satellite>, S>) -> u32 {
	solar_system.values().map(|sat| sat.count_orbits()).sum()
}
