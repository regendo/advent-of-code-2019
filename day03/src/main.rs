use day03::*;

fn main() {
	let (dirs_a, dirs_b) = read_directions("input.txt").unwrap();
	let (line_a, line_b) = (create_line(&dirs_a), create_line(&dirs_b));

	let intersections = find_intersection_distances(line_a, line_b);

	if let Some((point, distance)) = find_nearest_intersection(&intersections) {
		println!(
			"The nearest intersection occurs at {:?}, at a distance of {}",
			point, distance
		);
	}

	if let Some((point, steps)) = find_first_intersection(&intersections, &dirs_a, &dirs_b) {
		println!(
			"The first reached intersection is {:?}, after a combined {} steps.",
			point, steps
		);
	}
}
