use day09::{execute_step, load_program, Opcode, State};
use day11::Robot;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let mut program = load_program("input.txt", 0xFFFF)?;
	let mut robot: Robot = Default::default();
	robot.run(&mut program)
}
