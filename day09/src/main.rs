use day09::*;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
	let original_program = load_program("input.txt")?;

	let input = io::stdin();
	let mut program = original_program;
	execute_program(&mut program, input.lock(), io::stdout()).unwrap();

	Ok(())
}
