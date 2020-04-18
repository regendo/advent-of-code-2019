use day05::{execute_program, load_program};

pub fn execute_with_input(program: &[i32], input: &str) -> String {
	let input = input.as_bytes();
	let mut program = Vec::from(program);
	let mut output = Vec::new();

	execute_program(&mut program, input, &mut output).expect("Program execution failed!");

	String::from_utf8(output).expect("Uh-Oh! Not UTF-8")
}
