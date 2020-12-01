use std::convert::TryFrom;

mod phasing;
pub mod stutter;

pub(crate) fn str_to_digits(input: &str) -> Vec<u8> {
	input
		.trim()
		.chars()
		.map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
		.collect()
}

pub fn solve_1() {
	let signal = str_to_digits(include_str!("input.txt"));
	let base_pattern = vec![0, 1, 0, -1];
	let iterations = 100;

	let output = (0..iterations).fold(signal, |signal, _| {
		phasing::apply_phase(&signal, &base_pattern)
	});

	println!(
		"After 100 iterations, the signal starts with {:?}.",
		&output[..8]
	);
}

pub fn solve_2() {
	let signal = str_to_digits(include_str!("input.txt"));
	let base_pattern = vec![0, 1, 0, -1];
	let iterations = 100;
}
