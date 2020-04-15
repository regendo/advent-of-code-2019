use day02::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let mut program = load_program("input.txt")?;
	println!("Program loaded!");
	restore_to_alarm_state(&mut program);
	println!("Restored program to 1202 alarm state.");
	println!("Executing program...");
	execute_program(&mut program).unwrap();
	println!(
		"Program exited successfully. Memory at position 0 is {}.",
		program[0]
	);
	Ok(())
}
