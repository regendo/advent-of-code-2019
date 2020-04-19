use day08::{compute_checksum, read_input, split_into_layers_by_dimension};

fn main() {
	let data = read_input("input.txt");
	let image = split_into_layers_by_dimension(&data, 25, 6);

	println!("Checksum: {}", compute_checksum(&image));
}
