use std::fs;

type Layer<'a> = Vec<&'a [u8]>;
type Image<'a> = Vec<Layer<'a>>;

fn string_to_data(string: &str) -> Vec<u8> {
	string
		.trim()
		.split("")
		.filter(|s| !s.is_empty())
		.map(|s| s.parse::<u8>().unwrap())
		.collect()
}

pub fn read_input(path: &str) -> Vec<u8> {
	string_to_data(&fs::read_to_string(path).unwrap())
}

pub fn split_into_layers_by_dimension(data: &[u8], width: usize, height: usize) -> Image {
	let layers = data.len() / (width * height);
	let mut image = Vec::new();

	for layer in 0..layers {
		let off = layer * width * height;
		let mut image_layer = Vec::new();

		for row in 0..height {
			let start = off + row * width;
			let end = off + (row + 1) * width;
			image_layer.push(&data[start..end]);
		}

		image.push(image_layer);
	}

	image
}

#[allow(clippy::ptr_arg)]
fn count_digit(layer: &Layer, digit: u8) -> u32 {
	#[allow(clippy::naive_bytecount)]
	layer
		.iter()
		.map(|row| row.iter().filter(|px| **px == digit).count() as u32)
		.sum()
}

#[allow(clippy::ptr_arg)]
pub fn compute_checksum(image: &Image) -> u32 {
	let layer_to_check = image
		.iter()
		.min_by_key(|layer| count_digit(&layer, 0))
		.unwrap();

	count_digit(&layer_to_check, 1) * count_digit(&layer_to_check, 2)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_splits_the_sample_correctly() {
		let data = string_to_data("123456789012");
		let image = split_into_layers_by_dimension(&data, 3, 2);
		assert_eq!(image.len(), 2);
		for layer in image.iter() {
			assert_eq!(layer.len(), 2);
			for row in layer {
				assert_eq!(row.len(), 3);
			}
		}
		assert_eq!(image[0][0], &data[0..3]);
		assert_eq!(image[0][1], &data[3..6]);
		assert_eq!(image[1][0], &data[6..9]);
		assert_eq!(image[1][1], &data[9..12]);
	}
}
