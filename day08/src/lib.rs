use std::convert::TryFrom;
use std::fmt;
use std::fs;

type Layer<'a> = Vec<&'a [u8]>;
type LayeredImage<'a> = Vec<Layer<'a>>;

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

pub fn split_into_layers_by_dimension(data: &[u8], width: usize, height: usize) -> LayeredImage {
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
pub fn compute_checksum(image: &LayeredImage) -> u32 {
	let layer_to_check = image
		.iter()
		.min_by_key(|layer| count_digit(&layer, 0))
		.unwrap();

	count_digit(&layer_to_check, 1) * count_digit(&layer_to_check, 2)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Pixel {
	Black,
	White,
	Transparent,
}
impl Pixel {
	fn overlay(self, other: Pixel) -> Pixel {
		match self {
			Pixel::Black => self,
			Pixel::White => self,
			Pixel::Transparent => other,
		}
	}
}
impl TryFrom<u8> for Pixel {
	type Error = &'static str;
	fn try_from(val: u8) -> Result<Self, Self::Error> {
		match val {
			0 => Ok(Pixel::Black),
			1 => Ok(Pixel::White),
			2 => Ok(Pixel::Transparent),
			_ => Err("Invalid pixel value!"),
		}
	}
}
impl fmt::Display for Pixel {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let symbol = match self {
			Pixel::Black => "■",
			Pixel::White | Pixel::Transparent => "□",
		};
		write!(f, "{}", symbol)
	}
}

#[allow(clippy::ptr_arg)]
pub fn decode_image(image: &LayeredImage) -> Vec<Vec<Pixel>> {
	let height = image[0].len();
	let width = image[0][0].len();

	let mut decoded = Vec::new();
	for row in 0..height {
		let mut line = Vec::new();
		for col in 0..width {
			let mut px = Pixel::Transparent;
			for layer in image {
				px = px.overlay(Pixel::try_from(layer[row][col]).unwrap());
			}
			line.push(px);
		}
		decoded.push(line);
	}

	decoded
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
