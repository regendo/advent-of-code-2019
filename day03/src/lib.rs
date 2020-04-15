enum Direction {
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
