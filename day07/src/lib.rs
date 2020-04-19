use day05::execute_program;

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
/// let phases = [4,3,2,1,0];
/// let program = [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
/// let output = chain_amplifiers(&program, phases);
/// assert_eq!(output.lines().next(), Some("43210"));
/// ```
///
/// 2.
/// ```
/// # use day07::chain_amplifiers;
/// let phases = [0,1,2,3,4];
/// let program = [3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
/// let output = chain_amplifiers(&program, phases);
/// assert_eq!(output.lines().next(), Some("54321"));
/// ```
///
/// 3.
/// ```
/// # use day07::chain_amplifiers;
/// let phases = [1,0,4,3,2];
/// let program = [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
/// let output = chain_amplifiers(&program, phases);
/// assert_eq!(output.lines().next(), Some("65210"));
/// ```
pub fn chain_amplifiers(program: &[i32], phases: [u8; 5]) -> String {
	let mut input = String::from("0");
	for phase in phases.iter() {
		input = execute_with_input(program, &format!("{}\n{}", phase, input));
	}

	input
}

/// Iterator that generates all possible phase sequences.
///
/// ## Examples
///
/// Normal usage
///
/// ```
/// # use day07::Phases;
/// let mut it = Phases::new();
/// assert_eq!(it.next(), Some([0, 1, 2, 3, 4]));
/// ```
///
/// Last element
///
/// ```
/// # use day07::Phases;
/// let mut it = Phases::new();
/// assert_eq!(it.last(), Some([4, 3, 2, 1, 0]));
/// ```
///
/// Only 5! or 120 ways to arrange 5 numbers
///
/// ```
/// # use day07::Phases;
/// let mut it = Phases::new();
/// assert_eq!(it.count(), 120);
/// ```
pub struct Phases {
	permutation: [u8; 5],
	started: bool,
}
impl Phases {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Phases {
			permutation: [0, 1, 2, 3, 4],
			started: false,
		}
	}

	fn index_of_rightmost_pairwise_sorted(&self) -> Option<usize> {
		for k in (0..4).rev() {
			if self.permutation[k] < self.permutation[k + 1] {
				return Some(k);
			}
		}
		None
	}

	fn index_of_rightmost_larger(&self, k: usize) -> Option<usize> {
		for l in ((k + 1)..=4).rev() {
			if self.permutation[k] < self.permutation[l] {
				return Some(l);
			}
		}
		None
	}
}

impl Iterator for Phases {
	// each phase is one permutation of the numbers 0..=4.
	type Item = [u8; 5];

	fn next(&mut self) -> Option<Self::Item> {
		if !self.started {
			self.started = true;
			return Some(self.permutation);
		}
		if self.permutation == [4, 3, 2, 1, 0] {
			return None;
		}
		// generate the next permutation in lexicographical order
		// see also: https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
		let k = self.index_of_rightmost_pairwise_sorted()?;
		let l = self.index_of_rightmost_larger(k)?;
		self.permutation.swap(k, l);
		self.permutation[(k + 1)..=4].reverse();

		Some(self.permutation)
	}
}

/// For a program, find the phase settings that produce the highest output.
pub fn find_optimal_phases(program: &[i32]) -> Option<([u8; 5], i32)> {
	let mut max: Option<([u8; 5], i32)> = None;
	let permutations = Phases::new();
	for phases in permutations {
		let output = chain_amplifiers(program, phases)
			.lines()
			.next()
			.unwrap()
			.parse::<i32>()
			.unwrap();
		if let Some((_, value)) = max {
			if output > value {
				max = Some((phases, output));
			}
		} else {
			max = Some((phases, output));
		}
	}

	max
}
