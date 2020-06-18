#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point2D(pub usize, pub usize);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Vec2D(pub i32, pub i32);

impl Point2D {
	/// Compute distance (2D vector) between two points.
	/// # Example
	/// ```
	/// # use day10::{Point2D, Vec2D};
	/// let a = Point2D(4, 4);
	/// let b = Point2D(2, 5);
	/// assert_eq!(a.distance_to(b), Vec2D(-2, 1));
	/// ```
	pub fn distance_to(&self, other: Self) -> Vec2D {
		Vec2D(
			other.0 as i32 - self.0 as i32,
			other.1 as i32 - self.1 as i32,
		)
	}

	/// Return only the points that are visible from this point in 2D space.
	/// That is, they are not the same as this point and they are not behind another point that is blocking a direct line of sight.
	pub fn filter_visible(&self, others: &[Self]) -> Vec<Self> {
		let with_distances: Vec<(Point2D, Vec2D)> =
			others.iter().map(|p| (*p, self.distance_to(*p))).collect();
		with_distances
			.iter()
			.filter(|(p, d)| {
				p != self
					&& with_distances
						.iter()
						.all(|(_, dist)| !dist.is_multiple_of(*d))
			})
			.map(|(p, _)| *p)
			.collect::<Vec<Point2D>>()
	}

	/// Create a number of points from a 2D map where each `#` represents a point and the top left corner is the (0,0) coordinate.
	/// # Example
	/// ```
	/// # use day10::Point2D;
	/// let map = vec![
	/// 	".#..#",
	/// 	".....",
	/// 	"#####",
	/// 	"....#",
	/// 	"...##"
	/// 	];
	/// let points = Point2D::from_asteroid_map(map);
	/// assert_eq!(points, [
	/// 	Point2D(1, 0), Point2D(4, 0),
	/// 	Point2D(0, 2), Point2D(1, 2), Point2D(2, 2), Point2D(3, 2), Point2D(4, 2),
	/// 	Point2D(4, 3),
	/// 	Point2D(3, 4), Point2D(4, 4)
	/// ]);
	/// ```
	pub fn from_asteroid_map<'a, I>(map: I) -> Vec<Self>
	where
		I: IntoIterator<Item = &'a str>,
	{
		map.into_iter()
			.enumerate()
			.map(|(y, line)| {
				line
					.char_indices()
					.filter_map(move |(x, c)| if c == '#' { Some(Point2D(x, y)) } else { None })
			})
			.flatten()
			.collect()
	}

	/// Find the point that has the highest amounts of direct lines of sights to other points.
	pub fn with_highest_visibility(points: &[Self]) -> Option<Self> {
		points
			.iter()
			.max_by_key(|p| p.filter_visible(points).len())
			.map(|p| *p)
	}
}

impl Vec2D {
	/// Check if this vector is the same as another vector but stretched out.
	/// # Examples
	/// ```
	/// # use day10::Vec2D;
	/// assert_eq!(false,  Vec2D(2, 3).is_multiple_of(Vec2D(2, 3)));
	/// ```
	/// ```
	/// # use day10::Vec2D;
	/// assert_eq!(true,  Vec2D(4, 6).is_multiple_of(Vec2D(2, 3)));
	/// ```
	/// ```
	/// # use day10::Vec2D;
	/// assert_eq!(true,  Vec2D(3, 0).is_multiple_of(Vec2D(2, 0)));
	/// ```
	/// ```
	/// # use day10::Vec2D;
	/// assert_eq!(false, Vec2D(3, 1).is_multiple_of(Vec2D(2, 0)));
	/// ```
	/// ```
	/// # use day10::Vec2D;
	/// assert_eq!(false, Vec2D(4, 7).is_multiple_of(Vec2D(2, 3)));
	/// ```
	/// ```
	/// # use day10::Vec2D;
	/// assert_eq!(false, Vec2D(-4, 0).is_multiple_of(Vec2D(2, 0)));
	/// ```
	/// ```
	/// # use day10::Vec2D;
	/// assert_eq!(true,  Vec2D(-4, 0).is_multiple_of(Vec2D(-2, 0)));
	/// ```
	/// ```
	/// # use day10::Vec2D;
	/// assert_eq!(false, Vec2D(-4, -2).is_multiple_of(Vec2D(2, 1)));
	/// ```
	pub fn is_multiple_of(self, other: Self) -> bool {
		match (self, other) {
			(Vec2D(0, a), Vec2D(0, b)) => a.abs() > b.abs() && a.signum() == b.signum(),
			(Vec2D(a, 0), Vec2D(b, 0)) => a.abs() > b.abs() && a.signum() == b.signum(),
			(_, Vec2D(0, _)) => false,
			(_, Vec2D(_, 0)) => false,
			(Vec2D(ax, ay), Vec2D(bx, by)) => {
				let x_factor = ax as f32 / bx as f32;
				let y_factor = ay as f32 / by as f32;

				// comparing these floats for equality might be an issue
				x_factor > 1.0 && x_factor == y_factor
			}
		}
	}

	/// Calculate the angle from the top that a vector represents.
	/// # Examples
	/// ```
	/// # use day10::Vec2D;
	/// let arrow = Vec2D(0, -1); // straight up
	/// assert_eq!(arrow.angle(), 0.0);
	/// ```
	///
	/// ```
	/// # use day10::Vec2D;
	/// let arrow = Vec2D(0, 1); // straight down
	/// assert_eq!(arrow.angle(), 180.0);
	/// ```
	///
	/// ```
	/// # use day10::Vec2D;
	/// let arrow = Vec2D(-1, 0); // straight left
	/// assert_eq!(arrow.angle(), 90.0);
	/// ```
	///
	/// ```
	/// # use day10::Vec2D;
	/// let arrow = Vec2D(1, 1); // bottom right "corner"
	/// assert_eq!(arrow.angle(), 180.0 + 45.0);
	/// ```
	pub fn angle(self) -> f32 {
		// in my vectors, positive x is rightwards and positive y is downwards
		// in a normal coordinate system, positive y would be upwards instead
		let x = self.0 as f32;
		let y = -self.1 as f32;

		// the angle between two points is easily calculated with arctan2
		let angle = (y.atan2(x)
			// however, that is with 0 at the right, not the top
			+ std::f32::consts::FRAC_PI_2)
			// and in radians from -pi to +pi instead of angles
			* 180.0 / std::f32::consts::PI
			// which we want to go from 0 to 360 degrees
			+ 180.0;

		angle % 360.0
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Asteroid {
	location: Point2D,
	arrow: Vec2D,
	angle: f32,
}

struct DestructorLaser {
	_station: Point2D,
	asteroids: Vec<Asteroid>,
	current_target_index: Option<usize>,
}

impl DestructorLaser {
	pub fn new(station: Point2D, asteroids: &[Point2D]) -> Self {
		let mut asteroids = asteroids
			.iter()
			.filter_map(|a| {
				if *a == station {
					None
				} else {
					let arrow = station.distance_to(*a);
					Some(Asteroid {
						location: *a,
						arrow,
						angle: arrow.angle(),
					})
				}
			})
			.collect::<Vec<Asteroid>>();

		asteroids.sort_unstable_by(|a, b| {
			a.angle
				.partial_cmp(&b.angle)
				.map(|order| {
					order.then_with(|| {
						if a.arrow.is_multiple_of(b.arrow) {
							std::cmp::Ordering::Greater
						} else {
							std::cmp::Ordering::Less
						}
					})
				})
				.unwrap()
		});

		let current_target_index = if asteroids.is_empty() { None } else { Some(0) };

		Self {
			_station: station,
			asteroids,
			current_target_index,
		}
	}
}

impl Iterator for DestructorLaser {
	type Item = Asteroid;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(idx) = self.current_target_index {
			let destroyed = self.asteroids.remove(idx);
			self.asteroids.rotate_left(idx);

			self.current_target_index = self
				.asteroids
				.iter()
				.enumerate()
				.skip_while(|(_, target)| target.angle <= destroyed.angle)
				.next()
				.map(|(idx, _)| idx)
				.or(self.asteroids.first().map(|_| 0));

			Some(destroyed)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn filters_correct_asteroids() {
		let map = vec![".#..#", ".....", "#####", "....#", "...##"];
		let points = Point2D::from_asteroid_map(map);

		assert_eq!(
			points
				.iter()
				.map(|p| p.filter_visible(&points).len())
				.collect::<Vec<usize>>(),
			[7, 7, 6, 7, 7, 7, 5, 7, 8, 7]
		);
	}

	#[test]
	fn max_is_at_3_4() {
		let map = ".#..#
		                            .....
		                            #####
		                            ....#
		                            ...##"
			.split_whitespace();
		let points = &Point2D::from_asteroid_map(map);

		let selected = Point2D::with_highest_visibility(points);
		assert_eq!(Some(8), selected.map(|p| p.filter_visible(points).len()));
		assert_eq!(Some(Point2D(3, 4)), selected);
	}

	#[test]
	fn max_is_at_5_8() {
		let map = "......#.#.
		                            #..#.#....
		                            ..#######.
		                            .#.#.###..
		                            .#..#.....
		                            ..#....#.#
		                            #..#....#.
		                            .##.#..###
		                            ##...#..#.
											 .#....####"
			.split_whitespace();
		let points = &Point2D::from_asteroid_map(map);

		let selected = Point2D::with_highest_visibility(points);
		assert_eq!(Some(33), selected.map(|p| p.filter_visible(points).len()));
		assert_eq!(Some(Point2D(5, 8)), selected);
	}

	#[test]
	fn max_is_at_1_2() {
		let map = "#.#...#.#.
		                            .###....#.
		                            .#....#...
		                            ##.#.#.#.#
		                            ....#.#.#.
		                            .##..###.#
		                            ..#...##..
		                            ..##....##
		                            ......#...
		                            .####.###."
			.split_whitespace();
		let points = &Point2D::from_asteroid_map(map);

		let selected = Point2D::with_highest_visibility(points);
		assert_eq!(Some(35), selected.map(|p| p.filter_visible(points).len()));
		assert_eq!(Some(Point2D(1, 2)), selected);
	}

	#[test]
	fn max_is_at_6_3() {
		let map = ".#..#..###
		                            ####.###.#
		                            ....###.#.
		                            ..###.##.#
		                            ##.##.#.#.
		                            ....###..#
		                            ..#.#..#.#
		                            #..#.#.###
		                            .##...##.#
		                            .....#.#.."
			.split_whitespace();
		let points = &Point2D::from_asteroid_map(map);

		let selected = Point2D::with_highest_visibility(points);
		assert_eq!(Some(41), selected.map(|p| p.filter_visible(points).len()));
		assert_eq!(Some(Point2D(6, 3)), selected);
	}

	#[test]
	fn max_is_at_11_13() {
		let map = ".#..##.###...#######
		                            ##.############..##.
		                            .#.######.########.#
		                            .###.#######.####.#.
		                            #####.##.#.##.###.##
		                            ..#####..#.#########
		                            ####################
		                            #.####....###.#.#.##
		                            ##.#################
		                            #####.##.###..####..
		                            ..######..##.#######
		                            ####.##.####...##..#
		                            .#####..#.######.###
		                            ##...#.##########...
		                            #.##########.#######
		                            .####.#.###.###.#.##
		                            ....##.##.###..#####
		                            .#.#.###########.###
		                            #.#.#.#####.####.###
		                            ###.##.####.##.#..##"
			.split_whitespace();
		let points = &Point2D::from_asteroid_map(map);

		let selected = Point2D::with_highest_visibility(points);
		assert_eq!(Some(210), selected.map(|p| p.filter_visible(points).len()));
		assert_eq!(Some(Point2D(11, 13)), selected);
	}
}
