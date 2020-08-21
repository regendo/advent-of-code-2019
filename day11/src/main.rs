use day09::{execute_step, load_program, Opcode, State};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let mut program = load_program("input.txt", 0xFFFF)?;
	let mut idx = 0usize;
	let mut state = State::new();
	loop {
		match execute_step(&mut program, &mut idx, &mut state, reader, writer).unwrap() {
			Opcode::Halt => Ok(()),
			_ => unimplemented!(),
		}
	}
}
