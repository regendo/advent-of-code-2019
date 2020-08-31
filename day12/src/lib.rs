use once_cell::sync::OnceCell;
use regex::Regex;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::error::Error;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug)]
pub struct Pos(i32, i32, i32);

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

fn advance_time(system: &[&mut Moon]) {
	unimplemented!()
}

fn total_energy(system: &[&mut Moon]) -> i32 {
	system.iter().map(|m| m.total_energy()).sum()
}
