use day09::{self, execute_step};
use std::{collections::HashMap, fmt::Display, io};
use std::{convert::TryFrom, error::Error};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
	DrawTile((i32, i32), Tile),
	Score(i32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
	Empty,
	Wall,
	Block,
	HorizontalPaddle,
	Ball,
}

impl Display for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				// Empty and Block are double-width because so are all the emoji
				Tile::Empty => "  ",
				Tile::Ball => "⚽",
				Tile::Wall => "▮▮",
				Tile::Block => "🎁",
				Tile::HorizontalPaddle => "🏃",
			}
		)
	}
}

impl TryFrom<i32> for Tile {
	type Error = String;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
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

fn parse_output(raw_output: Vec<u8>) -> Result<Vec<Instruction>, Box<dyn Error>> {
	let text_output = String::from_utf8(raw_output)?;
	let mut sanitized_output = text_output
		.lines()
		.filter_map(|line| i32::from_str_radix(line.trim(), 10).ok())
		.peekable();

	let mut instructions = Vec::new();

	while let Some(_) = sanitized_output.peek() {
		match (
			sanitized_output.next(),
			sanitized_output.next(),
			sanitized_output.next(),
		) {
			(Some(-1), Some(0), Some(score)) => instructions.push(Instruction::Score(score)),
			(Some(x), Some(y), Some(code)) => {
				instructions.push(Instruction::DrawTile((x, y), Tile::try_from(code)?))
			}
			_ => Err("Leftover values!")?,
		}
	}

	Ok(instructions)
}

fn part_1() -> Result<(), Box<dyn Error>> {
	let mut program = day09::load_program("input.txt", 0xFFFF)?;

	let mut output = Vec::new();
	day09::execute_program(&mut program, io::empty(), &mut output)?;

	let mut canvas: HashMap<(i32, i32), Tile> = HashMap::new();
	let instructions = parse_output(output)?;
	instructions.iter().for_each(|instruction| {
		if let Instruction::DrawTile((x, y), tile) = instruction {
			*canvas.entry((*x, *y)).or_insert(Tile::Empty) = *tile;
		}
	});

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
	let mut canvas: HashMap<(i32, i32), Tile> = HashMap::new();
	let mut score = 0;
	let mut max_x = 9;
	let mut max_y = 9;

	loop {
		match day09::execute_step(&mut program, &mut idx, &mut state, &mut input, &mut output)? {
			day09::Opcode::Halt => break,
			day09::Opcode::Output => {
				if let Ok(instructions) = parse_output(output.clone()) {
					for instruction in instructions {
						match instruction {
							Instruction::Score(value) => score = value,
							Instruction::DrawTile((x, y), tile) => {
								*canvas.entry((x, y)).or_insert(Tile::Empty) = tile;
								if x > max_x {
									max_x = x;
								}
								if y > max_y {
									max_y = y;
								}
							}
						}
					}

					println!("Score: {}", score);
					for y in 0..=max_y {
						for x in 0..=max_x {
							print!(
								"{}",
								match canvas.get(&(x, y)) {
									Some(tile) => *tile,
									None => Tile::Empty,
								}
							)
						}
						println!();
					}
				}
			}
			_ => (),
		}
	}

	Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
	// part_1()?;
	part_2()?;

	Ok(())
}
