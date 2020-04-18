use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::hash::BuildHasher;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
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

	/// Compute the distance between one satellite that (indirectly) orbits another.
	fn orbit_distance(&self, target: Rc<Satellite>) -> Option<u32> {
		match &*self.orbit.borrow() {
			None => None,
			Some(other) if *other == target => Some(1),
			Some(other) => match other.orbit_distance(target) {
				None => None,
				Some(num) => Some(num + 1),
			},
		}
	}

	/// Find the closest orbit that two satellites share.
	///
	/// The function name was chosen for similarity with the greatest common denominator.
	fn greatest_common_orbit(&self, target: Rc<Satellite>) -> Option<Rc<Satellite>> {
		if self.orbit.borrow().is_none() {
			return None;
		}

		// So this recursive implementation works but if I try to do it iteratively, I can't reassign my variable because it's still borrowed
		if let Some(sat) = &*target.orbit.borrow() {
			if self.orbit_distance(Rc::clone(sat)).is_some() {
				return Some(Rc::clone(sat));
			} else {
				return self.greatest_common_orbit(Rc::clone(sat));
			}
		}

		None
	}

	pub fn transfers_between_orbits(&self, other: Rc<Satellite>) -> Option<u32> {
		if let Some(common_orbit) = self.greatest_common_orbit(Rc::clone(&other)) {
			let dist_a = self.orbit_distance(Rc::clone(&common_orbit));
			let dist_b = other.orbit_distance(common_orbit);
			dist_a.and_then(|a| dist_b.map(|b| a + b))
		} else {
			None
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

pub fn build_solar_system(input: &[(String, String)]) -> HashMap<String, Rc<Satellite>> {
	let mut solar_system = HashMap::<String, Rc<Satellite>>::new();

	for (orb, sat) in input {
		solar_system
			.entry((orb).to_string())
			.or_insert_with(|| Satellite::new(orb));
		solar_system
			.entry((sat).to_string())
			.or_insert_with(|| Satellite::new(sat));

		// we need to get it here instead of above through `or_insert_with` because that would mutably borrow
		let satellite = solar_system.get(sat).unwrap();
		*satellite.orbit.borrow_mut() = Some(Rc::clone(solar_system.get(orb).unwrap()));
	}

	solar_system
}

pub fn count_total_orbits<S: BuildHasher>(solar_system: &HashMap<String, Rc<Satellite>, S>) -> u32 {
	solar_system.values().map(|sat| sat.count_orbits()).sum()
}
