use day05::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let original_program = load_program("input.txt")?;

	let mut program = original_program.clone();
	println!("Program loaded!");
	restore_to_alarm_state(&mut program);
	println!("Restored program to 1202 alarm state.");
	println!("Executing program...");
	execute_program(&mut program).unwrap();
	println!(
		"Program exited successfully. Memory at position 0 is {}.",
		program[0]
	);

	let expected_output = 19_690_720;
	println!(
		"Attempting to find inputs to produce the output {}.",
		expected_output
	);
	if let Some(result) = find_correct_inputs(&original_program, expected_output) {
		println!("Input: {}", 100 * result.noun + result.verb);
	} else {
		println!("Could not find a matching input!")
	}
	Ok(())
}
