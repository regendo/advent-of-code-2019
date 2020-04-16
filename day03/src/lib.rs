use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

type Line = HashSet<Point>;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
	Left(u32),
	Right(u32),
	Up(u32),
	Down(u32),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
	x: i32,
	y: i32,
}

impl Point {
	/// ["Manhattan Distance"](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points.
	///
	/// Note that this distance is in absolute values and symmetrical.
	fn distance_to(self, other: Point) -> u32 {
		((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
	}

	/// Travel a distance from this point, returning all points traveled through.
	fn travel(self, direction: Direction) -> Vec<Point> {
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

fn map_code_to_dir(code: &str) -> Direction {
	let distance = code[1..].parse::<u32>().unwrap();
	// can't match on `code.starts_with(&str)` so here's this
	match code.chars().next() {
		Some('L') => Direction::Left(distance),
		Some('R') => Direction::Right(distance),
		Some('U') => Direction::Up(distance),
		Some('D') => Direction::Down(distance),
		_ => panic!("Unexpected code: {}", code),
	}
}

pub fn read_directions(path: &str) -> Result<(Vec<Direction>, Vec<Direction>), Box<dyn Error>> {
	let file = fs::read_to_string(path)?;
	let mut inputs = file.trim().lines().map(|l| {
		l.split(',')
			.map(map_code_to_dir)
			.collect::<Vec<Direction>>()
	});

	Ok((inputs.next().unwrap(), inputs.next().unwrap()))
}

pub fn create_line(directions: &[Direction]) -> Line {
	let mut line = Line::new();
	let mut current_location = Point { x: 0, y: 0 };
	// Inserting {0, 0} into our set might seem stupid because we'll filter it back out later
	// but we can't prevent it from being there anyway as we might visit it again during our travels.
	// So we might as well include it here to accurately present the line as every visited point.
	line.insert(current_location);

	for direction in directions {
		let traveled = current_location.travel(*direction);
		if let Some(destination) = traveled.last() {
			// update current location if we traveled anywhere
			// should be every time, but theoretically the travel distance could be 0
			current_location = *destination;
		}
		line.extend(traveled);
	}

	line
}

pub fn find_intersection_distances(a: Line, b: Line) -> HashMap<Point, u32> {
	let base = Point { x: 0, y: 0 };
	let intersections = a.intersection(&b).filter(|p| **p != base);
	let mut map = HashMap::new();

	for point in intersections {
		map.insert(*point, base.distance_to(*point));
	}

	map
}

pub fn find_nearest_intersection(intersections: HashMap<Point, u32>) -> Option<(Point, u32)> {
	if let Some((p, d)) = intersections.iter().min_by_key(|(_, distance)| *distance) {
		Some((*p, *d))
	} else {
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn spec_1() {
		let directions_a = ["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"]
			.iter()
			.map(|code| map_code_to_dir(*code))
			.collect::<Vec<_>>();
		let directions_b = ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"]
			.iter()
			.map(|code| map_code_to_dir(*code))
			.collect::<Vec<_>>();

		let line_a = create_line(&directions_a);
		let line_b = create_line(&directions_b);

		let (_, distance) =
			find_nearest_intersection(find_intersection_distances(line_a, line_b)).unwrap();

		assert_eq!(distance, 159);
	}
	#[test]
	fn spec_2() {
		let directions_a = [
			"R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
		]
		.iter()
		.map(|code| map_code_to_dir(*code))
		.collect::<Vec<_>>();

		let directions_b = [
			"U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
		]
		.iter()
		.map(|code| map_code_to_dir(*code))
		.collect::<Vec<_>>();

		let line_a = create_line(&directions_a);
		let line_b = create_line(&directions_b);

		let (_, distance) =
			find_nearest_intersection(find_intersection_distances(line_a, line_b)).unwrap();

		assert_eq!(distance, 135);
	}
}
