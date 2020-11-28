use std::{error::Error, sync::RwLock};

use day09 as intcode;
use day15 as lib;

fn main() -> Result<(), Box<dyn Error>> {
	let _game = lib::GameState::default();
	let game = RwLock::new(_game);
	let ai = lib::io::AI { game_state: &game };
	let output = lib::io::Output {
		game_state: &game,
		framecount: 0,
	};
	let mut program = intcode::load_program("input.txt", 0xFFFF)?;
	intcode::execute_program(&mut program, ai, output)?;

	println!("{}", game.read().unwrap());

	Ok(())
}
