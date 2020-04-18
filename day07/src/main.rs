use day05;
use day07::execute_with_input;

fn main() {
	let program = day05::load_program("../day05/input.txt").expect("Failed loading the program!");
	let output = execute_with_input(&program, "1");
	println!("Running with day 5's program with input `1`: {:?}", output);

	let output = execute_with_input(&program, "5");
	println!("Running with day 5's program with input `5`: {:?}", output);
}
