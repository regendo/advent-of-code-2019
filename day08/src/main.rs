use day08::{compute_checksum, decode_image, read_input, split_into_layers_by_dimension};

fn main() {
	let data = read_input("input.txt");
	let image = split_into_layers_by_dimension(&data, 25, 6);

	println!("Checksum: {}", compute_checksum(&image));

	let decoded = decode_image(&image);
	println!("Decoded to:");
	for line in decoded {
		println!(
			"{}",
			line.iter().map(|px| px.to_string()).collect::<String>()
		);
	}
}
