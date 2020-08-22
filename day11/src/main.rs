use day09;
use day11::Robot;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let mut program = day09::load_program("input.txt", 0xFFFF)?;
	let mut robot: Robot = Default::default();

	let panels_painted = robot.dry_run(&mut program.clone())?;
	println!("{} panels would be painted at least once!", panels_painted);

	robot = Default::default();
	let painting = robot.run(&mut program)?;
	println!("Painted on our hull:");
	println!("{}", painting);

	Ok(())
}
