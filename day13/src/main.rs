use day09::{self, execute_step};
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

fn part_1() -> Result<(), Box<dyn Error>> {
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
		"{} block tiles visible.",
		canvas.values().filter(|tile| **tile == Tile::Block).count()
	);

	Ok(())
}

fn part_2() -> Result<(), Box<dyn Error>> {
	let mut program = day09::load_program("input.txt", 0xFFFF)?;
	program[0] = 2;

	let mut output = Vec::new();
	let mut input = io::BufReader::new(io::stdin());
	let mut idx = 0_usize;
	let mut state = day09::State::new();

	loop {
		match day09::execute_step(&mut program, &mut idx, &mut state, &mut input, &mut output)? {
			day09::Opcode::Halt => break,
			day09::Opcode::Output => todo!(),
			_ => (),
		}
	}

	Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
	part_1()?;
	part_2()?;

	Ok(())
}
