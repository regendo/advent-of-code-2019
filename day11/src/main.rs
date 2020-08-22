use day09::{execute_step, load_program, Opcode, State};
use day11::Robot;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let mut program = load_program("input.txt", 0xFFFF)?;
	let mut robot: Robot = Default::default();

	let panels_painted = robot.dry_run(&mut program)?;
	println!("{} panels would be painted at least once!", panels_painted);

	Ok(())
}
