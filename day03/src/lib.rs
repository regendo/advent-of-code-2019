use std::error::Error;
use std::fs;

#[derive(Debug)]
pub enum Direction {
	Left(u32),
	Right(u32),
	Up(u32),
	Down(u32),
}

struct Point {
	x: i32,
	y: i32,
}

impl Point {
	/// ["Manhattan Distance"](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points.
	///
	/// Note that this distance is in absolute values and symmetrical.
	fn distance_to(&self, other: &Point) -> i32 {
		(self.x - other.x).abs() + (self.y - other.y).abs()
	}

	/// Travel a distance from this point, returning all points traveled through.
	fn travel(&self, direction: Direction) -> Vec<Point> {
		let distance = match direction {
			Direction::Left(d) | Direction::Right(d) | Direction::Up(d) | Direction::Down(d) => d,
		};
		let mut traveled = Vec::with_capacity(distance as usize);

		for i in 1..=(distance as i32) {
			#[rustfmt::skip]
			let bus_stop = match direction {
				Direction::Left(_)  => Point { x: self.x - i, y: self.y },
				Direction::Right(_) => Point { x: self.x + i, y: self.y },
				Direction::Up(_)    => Point { x: self.x,     y: self.y + i },
				Direction::Down(_)  => Point { x: self.x,     y: self.y - i },
			};

			traveled.push(bus_stop);
		}

		traveled
	}
}

pub fn read_directions(path: &str) -> Result<(Vec<Direction>, Vec<Direction>), Box<dyn Error>> {
	let mapper = |code: &str| -> Direction {
		let distance = code[1..].parse::<u32>().unwrap();
		// can't match on `code.starts_with(&str)` so here's this
		match code.chars().next() {
			Some('L') => Direction::Left(distance),
			Some('R') => Direction::Right(distance),
			Some('U') => Direction::Up(distance),
			Some('D') => Direction::Down(distance),
			_ => panic!("Unexpected code: {}", code),
		}
	};

	let file = fs::read_to_string(path)?;
	let mut inputs = file
		.trim()
		.lines()
		.map(|l| l.split(',').map(mapper).collect::<Vec<Direction>>());

	Ok((inputs.next().unwrap(), inputs.next().unwrap()))
}
