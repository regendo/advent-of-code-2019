use day03::*;

fn main() {
	let (dirs_a, dirs_b) = read_directions("input.txt").unwrap();
	let (line_a, line_b) = (create_line(&dirs_a), create_line(&dirs_b));

	if let Some((point, distance)) =
		find_nearest_intersection(find_intersection_distances(line_a, line_b))
	{
		println!(
			"The nearest intersection occurs at {:?}, at a distance of {}",
			point, distance
		);
	}
}
