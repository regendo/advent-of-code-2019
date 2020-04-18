use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

#[derive(Debug)]
struct Satellite {
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
}

fn read_input(path: &str) -> Vec<(String, String)> {
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

fn build_solar_system(input: &[(&str, &str)]) {
	let mut solar_system = HashMap::<&str, Rc<Satellite>>::new();

	for (orb, sat) in input {
		if !solar_system.contains_key(orb) {
			solar_system.insert(orb, Satellite::new(orb));
		}
		if !solar_system.contains_key(sat) {
			solar_system.insert(sat, Satellite::new(sat));
		}

		let satellite = solar_system.get(sat).unwrap();
		*satellite.orbit.borrow_mut() = Some(Rc::clone(solar_system.get(orb).unwrap()));
	}
}
