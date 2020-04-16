use day04::*;

fn main() {
	println!(
		"Found {} possible passwords that match our criteria.",
		count_valid_options()
	);
	println!(
		"With the new, more strict criteria, {} possible passwords remain.",
		count_valid_options_strictly()
	);
}
