use day05::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let original_program = load_program("input.txt")?;

	let mut program = original_program;
	execute_program(&mut program).unwrap();

	Ok(())
}
