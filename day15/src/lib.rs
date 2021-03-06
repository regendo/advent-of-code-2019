use ansi_term::Color;
use std::{collections::HashMap, convert::TryFrom, fmt::Display};

pub mod io;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Feedback {
	Moved,
	MovedAndFoundTarget,
	EncounteredWall,
}

impl TryFrom<u8> for Feedback {
	type Error = String;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		use Feedback::*;
		Ok(match value {
			0 => EncounteredWall,
			1 => Moved,
			2 => MovedAndFoundTarget,
			_ => return Err(format!("Invalid status code {}.", value)),
		})
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
	North,
	South,
	West,
	East,
}

impl Direction {
	fn to_code(self) -> u8 {
		use Direction::*;
		match self {
			North => 1,
			South => 2,
			West => 3,
			East => 4,
		}
	}

	fn step(self, pos: (i32, i32)) -> (i32, i32) {
		use Direction::*;
		match self {
			West => (pos.0 - 1, pos.1),
			East => (pos.0 + 1, pos.1),
			North => (pos.0, pos.1 - 1),
			South => (pos.0, pos.1 + 1),
		}
	}

	fn reverse(self) -> Self {
		use Direction::*;
		match self {
			North => South,
			South => North,
			West => East,
			East => West,
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
	Unexplored,
	Traversable,
	Wall,
	Droid,
	Cursed,
}

impl Display for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Tile::*;
		write!(
			f,
			"{}",
			match self {
				Unexplored => Color::Cyan.paint("?"),
				Traversable => Color::Black.paint("."),
				Wall => Color::White.paint("#"),
				Droid => Color::Red.paint("X"),
				Cursed => Color::White.dimmed().paint("#"),
			}
		)
	}
}

impl Default for Tile {
	fn default() -> Self {
		Self::Unexplored
	}
}

pub struct GameState {
	droid_starting_pos: (i32, i32),
	droid_pos: (i32, i32),
	world: HashMap<(i32, i32), Tile>,
	world_size: HashMap<Direction, u32>,
	previous_move: Option<Direction>,
	block_way_back: bool,
}

impl Default for GameState {
	fn default() -> Self {
		Self {
			droid_starting_pos: (0, 0),
			droid_pos: (0, 0),
			world: vec![((0, 0), Tile::Droid)].into_iter().collect(),
			world_size: vec![
				(Direction::North, 1),
				(Direction::South, 1),
				(Direction::East, 1),
				(Direction::West, 1),
			]
			.into_iter()
			.collect(),
			previous_move: None,
			block_way_back: false,
		}
	}
}

impl Display for GameState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for y in -(*self.world_size.get(&Direction::North).unwrap() as i32)
			..=(*self.world_size.get(&Direction::South).unwrap() as i32)
		{
			for x in -(*self.world_size.get(&Direction::West).unwrap() as i32)
				..=(*self.world_size.get(&Direction::East).unwrap() as i32)
			{
				match self.world.get(&(x, y)) {
					Some(Tile::Droid) => write!(f, "{}", Tile::Droid),
					Some(tile) => {
						if (x, y) == self.droid_starting_pos {
							write!(f, "{}", Color::Yellow.paint("S"))
						} else {
							write!(f, "{}", *tile)
						}
					}
					None => write!(f, "{}", Tile::default()),
				}?;
			}
			write!(f, "\n")?;
		}
		Ok(())
	}
}
