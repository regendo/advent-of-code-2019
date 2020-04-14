use std::fs;

enum Opcode {
	Add,
	Mult,
	Halt,
}

impl Opcode {
	fn new(code: i32) -> Result<Opcode, OpcodeError> {
		match code {
			1 => Ok(Opcode::Add),
			2 => Ok(Opcode::Mult),
			99 => Ok(Opcode::Halt),
			_ => Err(OpcodeError::UnknownOpcode(code)),
		}
	}
}

enum OpcodeError {
	UnknownOpcode(i32),
	TooFewParameters { expected: i32, actual: i32 },
	InvalidIndex(i32),
}

pub fn load_program(file_path: &str) -> Result<Vec<i32>, std::io::Error> {
	let file = fs::read_to_string(file_path)?;
	let program = file
		.trim()
		.split(',')
		.map(|s| s.parse::<i32>().unwrap())
		.collect();
	Ok(program)
}

fn execute_program(program: &mut [i32]) -> Result<(), OpcodeError> {
	// TODO
	Ok(())
}
