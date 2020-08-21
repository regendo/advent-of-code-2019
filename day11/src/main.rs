use day09::{execute_program, load_program};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let program = load_program("input.txt", 0xFFFF)?;
	execute_program(&mut program, reader, writer).unwrap();

	Ok(())
}
