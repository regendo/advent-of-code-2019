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
	pub fn distance_to(&self, other: Point2D) -> Vec2D {
		Vec2D(
			other.0 as i32 - self.0 as i32,
			other.1 as i32 - self.1 as i32,
		)
	}

	/// Return only the points that are visible from this point in 2D space.
	/// That is, they are not the same as this point and they are not behind another point that is blocking a direct line of sight.
	// pub fn filter_visible(&self, others: &[Point2D]) -> &[Point2D] {}

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
	/// 	Point2D(0, 1), Point2D(0, 4),
	/// 	Point2D(2, 0), Point2D(2, 1), Point2D(2, 2), Point2D(2, 3), Point2D(2, 4),
	/// 	Point2D(3, 4),
	/// 	Point2D(4, 3), Point2D(4, 4)
	/// ]);
	/// ```
	pub fn from_asteroid_map<'a, I>(map: I) -> Vec<Point2D>
	where
		I: IntoIterator<Item = &'a str>,
	{
		map.into_iter()
			.enumerate()
			.map(|(x, line)| {
				line
					.char_indices()
					.filter_map(move |(y, c)| if c == '#' { Some(Point2D(x, y)) } else { None })
			})
			.flatten()
			.collect()
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
	pub fn is_multiple_of(self, other: Vec2D) -> bool {
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
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	// #[test]
// }
