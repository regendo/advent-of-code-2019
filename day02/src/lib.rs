use std::fs;

enum Opcode {
	Add,
	Mult,
	Halt,
}

impl Opcode {
	fn new(code: usize) -> Result<Opcode, OpcodeError> {
		match code {
			1 => Ok(Opcode::Add),
			2 => Ok(Opcode::Mult),
			99 => Ok(Opcode::Halt),
			_ => Err(OpcodeError::UnknownOpcode(code)),
		}
	}
}

pub enum OpcodeError {
	UnknownOpcode(usize),
}

pub fn load_program(file_path: &str) -> Result<Vec<usize>, std::io::Error> {
	let file = fs::read_to_string(file_path)?;
	let program = file
		.trim()
		.split(',')
		.map(|s| s.parse::<usize>().unwrap())
		.collect();
	Ok(program)
}

/// Execute an Intcode program.
///
/// ## Examples
/// 1.
/// ```
/// # use day02::execute_program;
/// let mut program = [1,0,0,0,99];
/// execute_program(&mut program).unwrap();
/// assert_eq!(program, [2,0,0,0,99]);
/// ```
/// 2.
/// ```
/// # use day02::execute_program;
/// let mut program = [2,3,0,3,99];
/// execute_program(&mut program).unwrap();
/// assert_eq!(program, [2,3,0,6,99]);
/// ```
/// 3.
/// ```
/// # use day02::execute_program;
/// let mut program = [2,4,4,5,99,0];
/// execute_program(&mut program).unwrap();
/// assert_eq!(program, [2,4,4,5,99,9801]);
/// ```
/// 4.
/// ```
/// # use day02::execute_program;
/// let mut program = [1,1,1,4,99,5,6,0,99];
/// execute_program(&mut program).unwrap();
/// assert_eq!(program, [30,1,1,4,2,5,6,0,99]);
/// ```
/// 5.
/// ```
/// # use day02::execute_program;
/// let mut program = [1,9,10,3,2,3,11,0,99,30,40,50];
/// execute_program(&mut program).unwrap();
/// assert_eq!(program, [3500,9,10,70,2,3,11,0,99,30,40,50]);
/// ```
pub fn execute_program(program: &mut [usize]) -> Result<(), OpcodeError> {
	// TODO
	Ok(())
}
