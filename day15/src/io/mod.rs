use std::{convert::TryFrom, io, sync::RwLock};

use crate::{Direction, Feedback, GameState, Tile};

pub struct AI<'a> {
	pub game_state: &'a RwLock<GameState>,
}

impl AI<'_> {
	fn choose_direction(&self) -> Direction {
		// TODO
		Direction::West
	}
}

impl io::Read for AI<'_> {
	#[allow(unused_variables)]
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		unimplemented!()
	}
}

impl io::BufRead for AI<'_> {
	fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
		unimplemented!()
	}

	#[allow(unused_variables)]
	fn consume(&mut self, amt: usize) {
		unimplemented!()
	}

	fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
		let dir = self.choose_direction();
		let msg = format!("{}\n", dir.to_code());
		buf.push_str(&msg);
		self.game_state.write().unwrap().previous_move = Some(dir);
		Ok(msg.len())
	}
}

pub struct Output<'a> {
	pub game_state: &'a RwLock<GameState>,
}

impl io::Write for Output<'_> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		// This function is called with just a single character each time.
		// So in some cases, it won't be a character we're interested in
		// but we need to say we've written a character anyway.

		let text = String::from_utf8(buf.to_owned())
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
		let text = text.trim();

		if !text.is_empty() {
			let feedback = Feedback::try_from(
				u8::from_str_radix(text, 10)
					.map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?,
			)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

			match feedback {
				Feedback::Moved => self.step(),
				Feedback::MovedAndFoundTarget => {
					self.step();
					println!("WE DID IT!")
				}
				Feedback::EncounteredWall => self.register_wall_ahead(),
			}

			println!("{}", self.game_state.read().unwrap())
		}

		Ok(1)
	}

	fn flush(&mut self) -> io::Result<()> {
		unimplemented!()
	}
}

impl Output<'_> {
	fn register_wall_ahead(&self) {
		let current_position = self.game_state.read().unwrap().droid_pos;
		let wall_position = self
			.game_state
			.read()
			.unwrap()
			.previous_move
			.unwrap()
			.step(current_position);
		*self
			.game_state
			.write()
			.unwrap()
			.world
			.entry(wall_position)
			.or_default() = Tile::Wall;
	}

	fn step(&self) {
		let current_position = self.game_state.read().unwrap().droid_pos;
		let next_position = self
			.game_state
			.read()
			.unwrap()
			.previous_move
			.unwrap()
			.step(current_position);

		let mut game = self.game_state.write().unwrap();
		*game.world.entry(next_position).or_default() = Tile::Droid;
		*game.world.entry(current_position).or_default() = Tile::Traversable;
		game.droid_pos = next_position;
	}
}
