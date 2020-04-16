use std::fs;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Opcode {
	Add,
	Mult,
	Halt,
	Input,
	Output,
}

impl Opcode {
	fn new(code: u32) -> Result<Opcode, IntcodeError> {
		match code {
			1 => Ok(Opcode::Add),
			2 => Ok(Opcode::Mult),
			3 => Ok(Opcode::Input),
			4 => Ok(Opcode::Output),
			99 => Ok(Opcode::Halt),
			_ => Err(IntcodeError::UnknownOpcode(code)),
		}
	}

	fn param_count(&self) -> u32 {
		match self {
			Opcode::Add => 2,
			Opcode::Mult => 2,
			Opcode::Input => 1,
			Opcode::Output => 1,
			Opcode::Halt => 0,
		}
	}
}

#[derive(Debug)]
pub enum IntcodeError {
	UnknownOpcode(u32),
	UnknownParameterMode(u32),
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum ParameterMode {
	Position,
	Immediate,
}

impl ParameterMode {
	fn new(code: u32) -> Result<ParameterMode, IntcodeError> {
		match code % 10 {
			0 => Ok(ParameterMode::Position),
			1 => Ok(ParameterMode::Immediate),
			_ => Err(IntcodeError::UnknownParameterMode(code % 10)),
		}
	}
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
/// # use day05::execute_program;
/// let mut program = [1,0,0,0,99];
/// execute_program(&mut program).unwrap();
/// assert_eq!(program, [2,0,0,0,99]);
/// ```
/// 2.
/// ```
/// # use day05::execute_program;
/// let mut program = [2,3,0,3,99];
/// execute_program(&mut program).unwrap();
/// assert_eq!(program, [2,3,0,6,99]);
/// ```
/// 3.
/// ```
/// # use day05::execute_program;
/// let mut program = [2,4,4,5,99,0];
/// execute_program(&mut program).unwrap();
/// assert_eq!(program, [2,4,4,5,99,9801]);
/// ```
/// 4.
/// ```
/// # use day05::execute_program;
/// let mut program = [1,1,1,4,99,5,6,0,99];
/// execute_program(&mut program).unwrap();
/// assert_eq!(program, [30,1,1,4,2,5,6,0,99]);
/// ```
/// 5.
/// ```
/// # use day05::execute_program;
/// let mut program = [1,9,10,3,2,3,11,0,99,30,40,50];
/// execute_program(&mut program).unwrap();
/// assert_eq!(program, [3500,9,10,70,2,3,11,0,99,30,40,50]);
/// ```
pub fn execute_program(program: &mut [usize]) -> Result<(), IntcodeError> {
	let mut idx: usize = 0;

	while let Ok(code) = Opcode::new(program[idx] as u32) {
		match code {
			Opcode::Add => {
				add(program, idx);
				idx += 4;
			}
			Opcode::Mult => {
				mult(program, idx);
				idx += 4;
			}
			Opcode::Halt => return Ok(()),
		}
	}

	// This feels awkward but `while let` doesn't have an else clause.
	Err(IntcodeError::UnknownOpcode(program[idx] as u32))
}

/// Parse an instruction into its opcode and its respective parameter modes.
///
/// The last two digits are the opcode, the remaining are the parameter modes in reverse order.
///
/// ## Examples
/// ```
/// # use day05::{parse_instruction, Opcode, ParameterMode};
/// let (op, modes) = parse_instruction(1002)?;
/// assert_eq!(op, Opcode::Mult);
/// assert_eq!(modes, vec![ParameterMode::Position, ParameterMode::Immediate]);
/// ```
pub fn parse_instruction(instruction: u32) -> Result<(Opcode, Vec<ParameterMode>), IntcodeError> {
	let (op_num, mut par_num) = (instruction % 100, instruction / 100);
	let op = Opcode::new(op_num)?;
	let mut modes = Vec::with_capacity(op.param_count() as usize);
	for _ in 0..op.param_count() {
		modes.push(ParameterMode::new(par_num % 10)?);
		par_num /= 10;
	}

	Ok((op, modes))
}

/// Indirect Addition.
///
/// Add the values referenced from positions `idx+1` and `idx+2`, and store them the position referenced at `idx+3`.
///
/// ## Examples
///
/// ```
/// # use day05::add;
/// let mut program = [3, 1, 0, 1, 2];
/// add(&mut program, 1);
/// assert_eq!(program, [3, 1, 4, 1, 2]);
/// ```
pub fn add(program: &mut [usize], idx: usize) {
	let (adx, bdx, target_idx) = (program[idx + 1], program[idx + 2], program[idx + 3]);
	let (a, b) = (program[adx], program[bdx]);
	program[target_idx] = a + b;
}

/// Indirect Multiplication.
///
/// Multiply the values referenced from positions `idx+1` and `idx+2`, and store them the position referenced at `idx+3`.
///
/// ## Examples
///
/// ```
/// # use day05::mult;
/// let mut program = [3, 2, 0, 1, 2];
/// mult(&mut program, 1);
/// assert_eq!(program, [3, 2, 6, 1, 2]);
/// ```
pub fn mult(program: &mut [usize], idx: usize) {
	let (adx, bdx, target_idx) = (program[idx + 1], program[idx + 2], program[idx + 3]);
	let (a, b) = (program[adx], program[bdx]);
	program[target_idx] = a * b;
}

/// "[R]estore the [...] program [...] to the "1202 program alarm" state it had just before the last computer caught fire."
pub fn restore_to_alarm_state(program: &mut [usize]) {
	program[1] = 12;
	program[2] = 2;
}

pub struct Inputs {
	pub noun: usize,
	pub verb: usize,
}

/// Attempt to find a pair of inputs for addresses 1, 2 that produce the expected output.
pub fn find_correct_inputs(program: &[usize], expected: usize) -> Option<Inputs> {
	let mut instance = Vec::from(program);
	for noun in 0..100_usize {
		for verb in 0..100_usize {
			instance.copy_from_slice(program);
			instance[1] = noun;
			instance[2] = verb;
			if let Ok(()) = execute_program(&mut instance) {
				if instance[0] == expected {
					return Some(Inputs { noun, verb });
				}
			}
		}
	}

	None
}
