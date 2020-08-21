use std::collections::HashMap;
use std::error::Error;
use std::io;

#[derive(Copy, Clone, Debug)]
enum Direction {
	Up,
	Left,
	Down,
	Right,
}

impl Default for Direction {
	fn default() -> Self {
		Direction::Up
	}
}

impl Direction {
	fn rotate_left(&self) -> Self {
		match self {
			Direction::Up => Direction::Left,
			Direction::Left => Direction::Down,
			Direction::Down => Direction::Right,
			Direction::Right => Direction::Up,
		}
	}

	fn rotate_right(&self) -> Self {
		match self {
			Direction::Up => Direction::Right,
			Direction::Right => Direction::Down,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
		}
	}

	fn move_in_direction(&self, position: Position, distance: i32) -> Position {
		match self {
			Direction::Up => Position {
				x: position.x,
				y: position.y + distance,
			},
			Direction::Left => Position {
				x: position.x - distance,
				y: position.y,
			},
			Direction::Down => Position {
				x: position.x,
				y: position.y - distance,
			},
			Direction::Right => Position {
				x: position.x + distance,
				y: position.y,
			},
		}
	}
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Position {
	x: i32,
	y: i32,
}

impl Default for Position {
	fn default() -> Self {
		Self { x: 0, y: 0 }
	}
}

#[derive(Debug, Copy, Clone)]
enum Color {
	White,
	Black,
}

impl Color {
	fn from_code(n: u8) -> Self {
		match n {
			0 => Color::Black,
			1 => Color::White,
			_ => unimplemented!(),
		}
	}

	fn to_code(&self) -> u8 {
		match self {
			Color::Black => 0,
			Color::White => 1,
		}
	}
}

#[derive(Debug)]
struct Hull {
	panels: HashMap<Position, Color>,
}

impl Default for Hull {
	fn default() -> Self {
		Self {
			panels: Default::default(),
		}
	}
}

impl Hull {
	fn read_color(&self, position: &Position) -> Color {
		*self.panels.get(position).or(Some(&Color::Black)).unwrap()
	}

	fn paint(&mut self, position: Position, color: Color) {
		self.panels.insert(position, color);
	}
}

#[derive(Debug, Default)]
pub struct Robot {
	position: Position,
	direction: Direction,
	mind: day09::State,
	hull: Hull,
}

impl Robot {
	pub fn run(&mut self, program: &mut [i128]) -> Result<(), Box<dyn Error>> {
		let mut idx = 0usize;
		let mut mind_output: &mut [u8] = &mut vec![];

		loop {
			// Way too complicated to get it to accept our data.
			let slice: &[u8] = &[self.read_from_camera()];
			let mut mind_input = &mut io::BufReader::new(slice);

			match day09::execute_step(
				program,
				&mut idx,
				&mut self.mind,
				&mut mind_input,
				&mut mind_output,
			)? {
				day09::Opcode::Halt => break,
				day09::Opcode::Output => self.respond_to_instructions(),
				_ => (),
			}
		}
		Ok(())
	}

	fn read_from_camera(&self) -> u8 {
		self.hull.read_color(&self.position).to_code()
	}

	fn respond_to_instructions(&mut self) {
		unimplemented!()
	}

	fn move_forward(&mut self) {
		self.position = self.direction.move_in_direction(self.position, 1);
	}
}
