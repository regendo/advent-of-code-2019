use day05::{execute_program, load_program};

fn execute_with_input(program: &[i32], input: &str) -> String {
	let input = input.as_bytes();
	let mut program = Vec::from(program);
	let mut output = Vec::new();

	execute_program(&mut program, input, &mut output).expect("Program execution failed!");

	String::from_utf8(output).expect("Uh-Oh! Not UTF-8")
}

/// Chain multiple executions of a program together so that each output is piped to the next execution.
///
/// The first program starts with an input of `0`. Each program execution is run with one phase value.
///
/// ## Examples:
/// 1.
/// ```
/// # use day07::chain_amplifiers;
/// let phases = vec![4,3,2,1,0];
/// let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
/// let output = chain_amplifiers(&program, &phases);
/// assert_eq!(output.lines().next(), Some("43210"));
/// ```
///
/// 2.
/// ```
/// # use day07::chain_amplifiers;
/// let phases = vec![0,1,2,3,4];
/// let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
/// let output = chain_amplifiers(&program, &phases);
/// assert_eq!(output.lines().next(), Some("54321"));
/// ```
///
/// 3.
/// ```
/// # use day07::chain_amplifiers;
/// let phases = vec![1,0,4,3,2];
/// let program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
/// let output = chain_amplifiers(&program, &phases);
/// assert_eq!(output.lines().next(), Some("65210"));
/// ```
pub fn chain_amplifiers(program: &[i32], phases: &[u8]) -> String {
	let mut input = String::from("0");
	for phase in phases {
		input = execute_with_input(program, &format!("{}\n{}", phase, input));
	}

	input
}
