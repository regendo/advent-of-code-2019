use once_cell::sync::OnceCell;
use regex::Regex;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::error::Error;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pos(i32, i32, i32);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Vel(i32, i32, i32);

impl Default for Vel {
	fn default() -> Self {
		Self(0, 0, 0)
	}
}

impl Add<Vel> for Pos {
	type Output = Self;

	fn add(self, other: Vel) -> Self::Output {
		Pos(self.0 + other.0, self.1 + other.1, self.2 + other.2)
	}
}

impl AddAssign<Vel> for Pos {
	fn add_assign(&mut self, rhs: Vel) {
		*self = *self + rhs;
	}
}

impl Pos {
	fn regex() -> &'static Regex {
		static RE: OnceCell<Regex> = OnceCell::new();
		RE.get_or_init(|| Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap())
	}
}

impl TryFrom<&str> for Pos {
	type Error = Box<dyn Error>;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let captures = Pos::regex().captures(value).ok_or("Invalid syntax!")?;
		Ok(Pos(
			// remember that .get(0) is the full match
			captures.get(1).ok_or("No x parameter.")?.as_str().parse()?,
			captures.get(2).ok_or("No y parameter.")?.as_str().parse()?,
			captures.get(3).ok_or("No z parameter.")?.as_str().parse()?,
		))
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Moon {
	pos: Pos,
	vel: Vel,
}

impl Moon {
	pub fn new(pos: &Pos) -> Self {
		Self {
			pos: *pos,
			vel: Default::default(),
		}
	}

	pub fn attract(&mut self, other: &mut Self) {
		// Oof that's quite ugly!
		match self.pos.0.cmp(&other.pos.0) {
			Ordering::Greater => {
				self.vel.0 -= 1;
				other.vel.0 += 1;
			}
			Ordering::Less => {
				other.vel.0 -= 1;
				self.vel.0 += 1;
			}
			Ordering::Equal => (),
		}
		match self.pos.1.cmp(&other.pos.1) {
			Ordering::Greater => {
				self.vel.1 -= 1;
				other.vel.1 += 1;
			}
			Ordering::Less => {
				other.vel.1 -= 1;
				self.vel.1 += 1;
			}
			Ordering::Equal => (),
		}
		match self.pos.2.cmp(&other.pos.2) {
			Ordering::Greater => {
				self.vel.2 -= 1;
				other.vel.2 += 1;
			}
			Ordering::Less => {
				other.vel.2 -= 1;
				self.vel.2 += 1;
			}
			Ordering::Equal => (),
		}
	}

	pub fn travel(&mut self) {
		self.pos += self.vel;
	}

	fn potential_energy(self) -> i32 {
		self.pos.0 + self.pos.1 + self.pos.2
	}

	fn kinetic_energy(self) -> i32 {
		self.vel.0 + self.vel.1 + self.vel.2
	}

	pub fn total_energy(self) -> i32 {
		self.potential_energy() * self.kinetic_energy()
	}
}

pub fn advance_time(system: &mut [Moon]) -> Result<(), Box<dyn Error>> {
	let idx_pairs = PairwiseIterator::new(0, system.len() - 1);
	for (a, b) in idx_pairs {
		let (left, right) = system.split_at_mut(b);
		left
			.get_mut(a)
			.ok_or(format!("Invalid index a: {}", a))?
			// this is 0 because this slice starts at `b`
			.attract(right.get_mut(0).ok_or(format!("Invalid index b: {}", b))?);
	}
	for moon in system.iter_mut() {
		(*moon).travel();
	}
	Ok(())
}

pub fn total_energy(system: &[&mut Moon]) -> i32 {
	system.iter().map(|m| m.total_energy()).sum()
}

#[derive(Debug)]
struct PairwiseIterator {
	cur_low: usize,
	cur_high: usize,
	abs_low: usize,
	abs_high: usize,
}

impl PairwiseIterator {
	fn new(a: usize, b: usize) -> Self {
		let (abs_low, abs_high) = match a.cmp(&b) {
			Ordering::Greater => (b, a),
			Ordering::Less => (a, b),
			Ordering::Equal => (a, a),
		};
		Self {
			abs_high,
			abs_low,
			cur_low: abs_low,
			// We never return this wrong cur_high value
			// because we always increment it before returning a Some.
			cur_high: abs_low,
		}
	}
}

impl Iterator for PairwiseIterator {
	type Item = (usize, usize);

	fn next(&mut self) -> Option<Self::Item> {
		match (self.cur_low, self.cur_high) {
			(a, b) if b == self.abs_high && b == a + 1 => None,
			(_, b) if b == self.abs_high => {
				self.cur_low += 1;
				self.cur_high = self.cur_low + 1;
				Some((self.cur_low, self.cur_high))
			}
			(_, b) if b < self.abs_high => {
				self.cur_high += 1;
				Some((self.cur_low, self.cur_high))
			}
			_ => unreachable!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_a() {
		let starting_positions = [Pos(-1, 0, 2), Pos(2, -10, -7), Pos(4, -8, 8), Pos(3, 5, -1)];
		let mut system: Vec<Moon> = starting_positions.iter().map(Moon::new).collect();

		advance_time(&mut system);
		assert_eq!(
			system,
			[
				Moon {
					pos: Pos(2, -1, 1),
					vel: Vel(3, -1, -1)
				},
				Moon {
					pos: Pos(3, -7, -4),
					vel: Vel(1, 3, 3)
				},
				Moon {
					pos: Pos(1, -7, 5),
					vel: Vel(-3, 1, -3)
				},
				Moon {
					pos: Pos(2, 2, 0),
					vel: Vel(-1, -3, 1)
				}
			]
		);
		for _ in 2..=5 {
			advance_time(&mut system);
		}
		assert_eq!(
			system,
			[
				Moon {
					pos: Pos(-1, -9, 2),
					vel: Vel(-3, -1, 2)
				},
				Moon {
					pos: Pos(4, 1, 5),
					vel: Vel(2, 0, -2)
				},
				Moon {
					pos: Pos(2, 2, -4),
					vel: Vel(0, -1, 2)
				},
				Moon {
					pos: Pos(3, -7, -1),
					vel: Vel(1, 2, -2)
				}
			]
		);
		for _ in 6..=10 {
			advance_time(&mut system);
		}
		assert_eq!(
			system,
			[
				Moon {
					pos: Pos(2, 1, -3),
					vel: Vel(-3, -2, 1)
				},
				Moon {
					pos: Pos(1, -8, 0),
					vel: Vel(-1, 1, 3)
				},
				Moon {
					pos: Pos(3, -6, 1),
					vel: Vel(3, 2, -3)
				},
				Moon {
					pos: Pos(2, 0, 4),
					vel: Vel(1, -1, -1)
				}
			]
		);
	}
}
