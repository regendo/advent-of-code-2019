#[derive(Debug, PartialEq, Eq)]
pub struct Point2D(pub usize, pub usize);

impl Point2D {
	/// Return only the points that are visible from this point in 2D space.
	/// That is, they are not the same as this point and they are not behind another point that is blocking a direct line of sight.
	// pub fn filter_visible(&self, others: &[Point2D]) -> &[Point2D] {
	// }

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

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	// #[test]
// }
