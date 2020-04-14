use day02::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let program = load_program("input.txt")?;
	println!("Loaded the program:\n{:?}", program);
	Ok(())
}
