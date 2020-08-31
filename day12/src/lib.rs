use once_cell::sync::OnceCell;
use regex::Regex;
use std::convert::TryFrom;
use std::error::Error;
use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub struct Pos(i32, i32, i32);

#[derive(Copy, Clone, Debug)]
struct Vel(i32, i32, i32);

impl Add<Vel> for Pos {
	type Output = Self;

	fn add(self, other: Vel) -> Pos {
		Pos(self.0 + other.0, self.1 + other.1, self.2 + other.2)
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
