use day09;
use std::u8;
use std::{collections::HashMap, io};
use std::{convert::TryFrom, error::Error};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
	Empty,
	Wall,
	Block,
	HorizontalPaddle,
	Ball,
}

impl TryFrom<u8> for Tile {
	type Error = String;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Tile::Empty),
			1 => Ok(Tile::Wall),
			2 => Ok(Tile::Block),
			3 => Ok(Tile::HorizontalPaddle),
			4 => Ok(Tile::Ball),
			_ => Err(format!("Unexpected tile value {}", value)),
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut program = day09::load_program("input.txt", 0xFFFF)?;

	let mut output = Vec::new();
	day09::execute_program(&mut program, io::empty(), &mut output)?;

	let text_output = String::from_utf8(output)?;
	let mut instructions = text_output
		.lines()
		.filter_map(|line| u8::from_str_radix(line.trim(), 10).ok())
		.peekable();

	let mut canvas: HashMap<(u8, u8), Tile> = HashMap::new();
	while let Some(_) = instructions.peek() {
		match (
			instructions.next(),
			instructions.next(),
			instructions.next(),
		) {
			(Some(x), Some(y), Some(code)) => {
				*canvas.entry((x, y)).or_insert(Tile::Empty) = Tile::try_from(code)?;
			}
			_ => panic!("Leftover elements!"),
		}
	}

	println!(
		"{} tiles visible.",
		canvas.values().filter(|tile| **tile != Tile::Empty).count()
	);

	Ok(())
}
