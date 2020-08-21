use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{BufRead, Write};

pub struct State {
	relative_base: i128,
}
impl State {
	pub fn new() -> Self {
		Default::default()
	}
}
impl Default for State {
	fn default() -> Self {
		Self { relative_base: 0 }
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Opcode {
	Add,
	Mult,
	Halt,
	Input,
	Output,
	JumpZero,
	JumpNonZero,
	CompareEq,
	CompareLt,
	AdjustRelBase,
}

impl Opcode {
	fn new(code: u128) -> Result<Opcode, IntcodeError> {
		match code {
			1 => Ok(Opcode::Add),
			2 => Ok(Opcode::Mult),
			3 => Ok(Opcode::Input),
			4 => Ok(Opcode::Output),
			5 => Ok(Opcode::JumpNonZero),
			6 => Ok(Opcode::JumpZero),
			7 => Ok(Opcode::CompareLt),
			8 => Ok(Opcode::CompareEq),
			9 => Ok(Opcode::AdjustRelBase),
			99 => Ok(Opcode::Halt),
			_ => Err(IntcodeError::UnknownOpcode(code)),
		}
	}

	fn param_count(self) -> usize {
		match self {
			Opcode::Add => 3,
			Opcode::Mult => 3,
			Opcode::Input => 1,
			Opcode::Output => 1,
			Opcode::Halt => 0,
			Opcode::JumpNonZero => 2,
			Opcode::JumpZero => 2,
			Opcode::CompareEq => 3,
			Opcode::CompareLt => 3,
			Opcode::AdjustRelBase => 1,
		}
	}
}

#[derive(Debug)]
pub enum IntcodeError {
	UnknownOpcode(u128),
	UnknownParameterMode(u128),
	ExcessiveParameterModes(u128),
	NegativeInstructionValue(i128),
	InvalidAddress(i128),
	TooFewParameterModes,
	WrongParameterMode,
}

impl fmt::Display for IntcodeError {
	// Required for Error but we really don't care.
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl Error for IntcodeError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		None
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ParameterMode {
	Position,
	Immediate,
	Relative,
}

impl ParameterMode {
	fn new(code: u128) -> Result<ParameterMode, IntcodeError> {
		match code % 10 {
			0 => Ok(ParameterMode::Position),
			1 => Ok(ParameterMode::Immediate),
			2 => Ok(ParameterMode::Relative),
			_ => Err(IntcodeError::UnknownParameterMode(code % 10)),
		}
	}
}

pub fn load_program(file_path: &str, memory_size: usize) -> Result<Vec<i128>, std::io::Error> {
	let file = fs::read_to_string(file_path)?;
	let mut program = file
		.trim()
		.split(',')
		.map(|s| s.parse::<i128>().unwrap())
		.collect::<Vec<i128>>();
	program.resize(memory_size, 0);
	Ok(program)
}

/// Execute an Intcode program.
///
/// ## Examples
/// 1.
/// ```
/// # use day09::execute_program;
/// let mut program = [1,0,0,0,99];
/// let (input, output) = ("".as_bytes(), vec![]);
/// execute_program(&mut program, input, output).unwrap();
/// assert_eq!(program, [2,0,0,0,99]);
/// ```
/// 2.
/// ```
/// # use day09::execute_program;
/// let mut program = [2,3,0,3,99];
/// let (input, output) = ("".as_bytes(), vec![]);
/// execute_program(&mut program, input, output).unwrap();
/// assert_eq!(program, [2,3,0,6,99]);
/// ```
/// 3.
/// ```
/// # use day09::execute_program;
/// let mut program = [2,4,4,5,99,0];
/// let (input, output) = ("".as_bytes(), vec![]);
/// execute_program(&mut program, input, output).unwrap();
/// assert_eq!(program, [2,4,4,5,99,9801]);
/// ```
/// 4.
/// ```
/// # use day09::execute_program;
/// let mut program = [1,1,1,4,99,5,6,0,99];
/// let (input, output) = ("".as_bytes(), vec![]);
/// execute_program(&mut program, input, output).unwrap();
/// assert_eq!(program, [30,1,1,4,2,5,6,0,99]);
/// ```
/// 5.
/// ```
/// # use day09::execute_program;
/// let mut program = [1,9,10,3,2,3,11,0,99,30,40,50];
/// let (input, output) = ("".as_bytes(), vec![]);
/// execute_program(&mut program, input, output).unwrap();
/// assert_eq!(program, [3500,9,10,70,2,3,11,0,99,30,40,50]);
/// ```
/// 6.
/// ```
/// # use day09::execute_program;
/// let original_program = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
/// let mut program = Vec::new();
/// program.extend_from_slice(&original_program);
/// program.resize(0xFF, 0);
/// let (input, mut output) = ("".as_bytes(), vec![]);
/// execute_program(&mut program, input, &mut output).unwrap();
/// let output = String::from_utf8(output)
/// 	.unwrap()
/// 	.trim()
/// 	.split('\n')
/// 	.filter(|s| !s.is_empty())
/// 	.map(|s| s.trim().parse::<i128>().unwrap())
/// 	.collect::<Vec<i128>>();
/// assert_eq!(output, original_program);
/// ```
/// 7.
/// ```
/// # use day09::execute_program;
/// let mut program = [1102,34915192,34915192,7,4,7,99,0];
/// let (input, mut output) = ("".as_bytes(), vec![]);
/// execute_program(&mut program, input, &mut output).unwrap();
/// let output = String::from_utf8(output).unwrap();
/// assert_eq!(output.trim().len(), 16);
/// ```
/// 8.
/// ```
/// # use day09::execute_program;
/// let large_number = 1_125_899_906_842_624;
/// let mut program = [104, large_number ,99];
/// let (input, mut output) = ("".as_bytes(), vec![]);
/// execute_program(&mut program, input, &mut output).unwrap();
/// let output = String::from_utf8(output).unwrap();
/// assert_eq!(output.trim(), large_number.to_string());
/// ```
pub fn execute_program<R, W>(
	program: &mut [i128],
	mut reader: R,
	mut writer: W,
) -> Result<(), IntcodeError>
where
	R: BufRead,
	W: Write,
{
	let mut idx: usize = 0;
	let mut state = State::new();

	loop {
		if let Opcode::Halt = execute_step(program, &mut idx, &mut state, &mut reader, &mut writer)? {
			return Ok(());
		};
	}
}

pub fn execute_step<R, W>(
	program: &mut [i128],
	idx: &mut usize,
	state: &mut State,
	reader: &mut R,
	writer: &mut W,
) -> Result<Opcode, IntcodeError>
where
	R: BufRead,
	W: Write,
{
	let prev_idx = *idx;
	let instruction = program[*idx];
	if instruction < 0 {
		return Err(IntcodeError::NegativeInstructionValue(instruction));
	}

	let (opcode, modes) = parse_instruction(instruction as u128)?;
	match opcode {
		Opcode::Add => add(program, *idx, &modes, &state)?,
		Opcode::Mult => mult(program, *idx, &modes, &state)?,
		Opcode::Input => input(program, *idx, &modes, reader, &state)?,
		Opcode::Output => output(program, *idx, &modes, writer, &state)?,
		Opcode::Halt => (),
		Opcode::CompareEq => compare_eq(program, *idx, &modes, &state)?,
		Opcode::CompareLt => compare_lt(program, *idx, &modes, &state)?,
		Opcode::JumpZero => jump_zero(program, idx, &modes, &state)?,
		Opcode::JumpNonZero => jump_non_zero(program, idx, &modes, &state)?,
		Opcode::AdjustRelBase => adjust_relative_base(program, *idx, &modes, state)?,
	}
	if prev_idx == *idx {
		// don't move our instruction pointer if we jumped
		*idx += 1 + opcode.param_count() as usize;
	}
	Ok(opcode)
}

/// Parse an instruction into its opcode and its respective parameter modes.
///
/// The last two digits are the opcode, the remaining are the parameter modes in reverse order.
///
/// ## Examples
/// ```
/// # use day09::{parse_instruction, Opcode, ParameterMode};
/// let (op, modes) = parse_instruction(1002).unwrap();
/// assert_eq!(op, Opcode::Mult);
/// assert_eq!(modes, vec![ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Position]);
/// ```
pub fn parse_instruction(instruction: u128) -> Result<(Opcode, Vec<ParameterMode>), IntcodeError> {
	let (op_num, mut par_num) = (instruction % 100, instruction / 100);
	let op = Opcode::new(op_num)?;
	let mut modes = Vec::with_capacity(op.param_count() as usize);
	for _ in 0..op.param_count() {
		modes.push(ParameterMode::new(par_num % 10)?);
		par_num /= 10;
	}

	if par_num > 0 {
		return Err(IntcodeError::ExcessiveParameterModes(par_num));
	}

	Ok((op, modes))
}

fn parse_parameter(
	param: i128,
	mode: Option<&ParameterMode>,
	program: &[i128],
	state: &State,
) -> Result<i128, IntcodeError> {
	match mode {
		Some(ParameterMode::Immediate) => Ok(param),
		Some(ParameterMode::Position) | Some(ParameterMode::Relative) => {
			match parse_address_parameter(param, mode, state) {
				Ok(pos) => Ok(program[pos]),
				Err(e) => Err(e),
			}
		}
		None => Err(IntcodeError::TooFewParameterModes),
	}
}

fn parse_address_parameter(
	param: i128,
	mode: Option<&ParameterMode>,
	state: &State,
) -> Result<usize, IntcodeError> {
	match mode {
		Some(ParameterMode::Position) => {
			if param < 0 {
				Err(IntcodeError::InvalidAddress(param))
			} else {
				Ok(param as usize)
			}
		}
		Some(ParameterMode::Relative) => {
			let pos = param + state.relative_base;
			if pos < 0 {
				Err(IntcodeError::InvalidAddress(pos))
			} else {
				Ok(pos as usize)
			}
		}
		Some(ParameterMode::Immediate) => Err(IntcodeError::WrongParameterMode),
		None => Err(IntcodeError::TooFewParameterModes),
	}
}

fn parse_jump_parameter(
	param: i128,
	mode: Option<&ParameterMode>,
	program: &[i128],
	state: &State,
) -> Result<usize, IntcodeError> {
	match parse_parameter(param, mode, program, state) {
		Ok(n) if n < 0 => Err(IntcodeError::InvalidAddress(n)),
		Ok(n) => Ok(n as usize),
		Err(e) => Err(e),
	}
}

/// Addition.
///
/// ## Examples
///
/// ```
/// # use day09::{add, parse_instruction, State};
/// let mut program = [3, 1, 0, 1, 2];
/// let state = State::new();
/// let idx = 1;
/// let (_, modes) = parse_instruction(program[idx] as u128).unwrap();
///
/// add(&mut program, idx, &modes, &state).unwrap();
/// assert_eq!(program, [3, 1, 4, 1, 2]);
/// ```
pub fn add(
	program: &mut [i128],
	idx: usize,
	modes: &[ParameterMode],
	state: &State,
) -> Result<(), IntcodeError> {
	let (param_a, param_b, param_target) = (program[idx + 1], program[idx + 2], program[idx + 3]);
	let mut modes = modes.iter();

	let a = parse_parameter(param_a, modes.next(), program, state)?;
	let b = parse_parameter(param_b, modes.next(), program, state)?;
	let target = parse_address_parameter(param_target, modes.next(), state)?;
	program[target] = a + b;
	Ok(())
}

/// Multiplication.
///
/// ## Examples
///
/// ```
/// # use day09::{mult, parse_instruction, State};
/// let mut program = [3, 2, 0, 1, 2];
/// let state = State::new();
/// let idx = 1;
/// let (_, modes) = parse_instruction(program[idx] as u128).unwrap();
///
/// mult(&mut program, idx, &modes, &state).unwrap();
/// assert_eq!(program, [3, 2, 6, 1, 2]);
/// ```
pub fn mult(
	program: &mut [i128],
	idx: usize,
	modes: &[ParameterMode],
	state: &State,
) -> Result<(), IntcodeError> {
	let (param_a, param_b, param_target) = (program[idx + 1], program[idx + 2], program[idx + 3]);
	let mut modes = modes.iter();

	let a = parse_parameter(param_a, modes.next(), program, state)?;
	let b = parse_parameter(param_b, modes.next(), program, state)?;
	let target = parse_address_parameter(param_target, modes.next(), state)?;
	program[target] = a * b;
	Ok(())
}

pub fn output<W>(
	program: &mut [i128],
	idx: usize,
	modes: &[ParameterMode],
	mut writer: W,
	state: &State,
) -> Result<(), IntcodeError>
where
	W: Write,
{
	let param_a = program[idx + 1];
	let mut modes = modes.iter();

	let a = parse_parameter(param_a, modes.next(), program, state)?;
	writeln!(&mut writer, "{}", a).expect("Can't write to output!");
	Ok(())
}

pub fn input<R>(
	program: &mut [i128],
	idx: usize,
	modes: &[ParameterMode],
	mut reader: R,
	state: &State,
) -> Result<(), IntcodeError>
where
	R: BufRead,
{
	let param_target = program[idx + 1];
	let mut modes = modes.iter();

	let target = parse_address_parameter(param_target, modes.next(), state)?;
	let mut input = String::new();
	reader.read_line(&mut input).unwrap();
	let num = input.trim().parse::<i128>().unwrap();

	program[target] = num;
	Ok(())
}

pub fn compare_eq(
	program: &mut [i128],
	idx: usize,
	modes: &[ParameterMode],
	state: &State,
) -> Result<(), IntcodeError> {
	let (param_a, param_b, param_target) = (program[idx + 1], program[idx + 2], program[idx + 3]);
	let mut modes = modes.iter();

	let a = parse_parameter(param_a, modes.next(), program, state)?;
	let b = parse_parameter(param_b, modes.next(), program, state)?;
	let target = parse_address_parameter(param_target, modes.next(), state)?;
	program[target] = if a == b { 1 } else { 0 };
	Ok(())
}

pub fn compare_lt(
	program: &mut [i128],
	idx: usize,
	modes: &[ParameterMode],
	state: &State,
) -> Result<(), IntcodeError> {
	let (param_a, param_b, param_target) = (program[idx + 1], program[idx + 2], program[idx + 3]);
	let mut modes = modes.iter();

	let a = parse_parameter(param_a, modes.next(), program, state)?;
	let b = parse_parameter(param_b, modes.next(), program, state)?;
	let target = parse_address_parameter(param_target, modes.next(), state)?;
	program[target] = if a < b { 1 } else { 0 };
	Ok(())
}

pub fn jump_zero(
	program: &mut [i128],
	idx: &mut usize,
	modes: &[ParameterMode],
	state: &State,
) -> Result<(), IntcodeError> {
	let (param_a, param_target) = (program[*idx + 1], program[*idx + 2]);
	let mut modes = modes.iter();

	let a = parse_parameter(param_a, modes.next(), program, state)?;
	let target = parse_jump_parameter(param_target, modes.next(), program, state)?;
	if a == 0 {
		*idx = target;
	}
	Ok(())
}

pub fn jump_non_zero(
	program: &mut [i128],
	idx: &mut usize,
	modes: &[ParameterMode],
	state: &State,
) -> Result<(), IntcodeError> {
	let (param_a, param_target) = (program[*idx + 1], program[*idx + 2]);
	let mut modes = modes.iter();

	let a = parse_parameter(param_a, modes.next(), program, state)?;
	let target = parse_jump_parameter(param_target, modes.next(), program, state)?;
	if a != 0 {
		*idx = target;
	}
	Ok(())
}

fn adjust_relative_base(
	program: &[i128],
	idx: usize,
	modes: &[ParameterMode],
	state: &mut State,
) -> Result<(), IntcodeError> {
	let param = program[idx + 1];
	let mut modes = modes.iter();

	let adjustment = parse_parameter(param, modes.next(), program, state)?;
	state.relative_base += adjustment;
	Ok(())
}

/// "Restore the [...] program [...] to the "1202 program alarm" state it had just before the last computer caught fire."
pub fn restore_to_alarm_state(program: &mut [i128]) {
	program[1] = 12;
	program[2] = 2;
}

pub struct Inputs {
	pub noun: i128,
	pub verb: i128,
}

/// Attempt to find a pair of inputs for addresses 1, 2 that produce the expected output.
pub fn find_correct_inputs<R, W>(
	program: &[i128],
	expected: i128,
	mut reader: R,
	mut writer: W,
) -> Option<Inputs>
where
	R: BufRead,
	W: Write,
{
	let mut instance = Vec::from(program);
	for noun in 0..100 {
		for verb in 0..100 {
			instance.copy_from_slice(program);
			instance[1] = noun;
			instance[2] = verb;
			if let Ok(()) = execute_program(&mut instance, &mut reader, &mut writer) {
				if instance[0] == expected {
					return Some(Inputs { noun, verb });
				}
			}
		}
	}

	None
}
