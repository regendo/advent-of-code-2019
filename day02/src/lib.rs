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

fn execute_program(program: &mut [i32]) -> Result<(), OpcodeError> {
	// TODO
	Ok(())
}
