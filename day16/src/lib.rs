use std::convert::TryFrom;

mod phasing;

pub(crate) fn str_to_digits(input: &str) -> Vec<u8> {
	input
		.trim()
		.chars()
		.map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
		.collect()
}

pub fn solve_1() {
	let signal = str_to_digits(include_str!("input.txt"));
	let iterations = 100;

	let output = (0..iterations).fold(signal, |signal, _| phasing::apply_phase(&signal));

	println!(
		"After 100 iterations, the signal starts with {:?}.",
		&output[..8]
	);
}

pub fn solve_2() {
	let signal = str_to_digits(include_str!("input.txt"));
	let iterations = 100;

	let output = (0..iterations).fold(signal, |signal, _| {
		phasing::apply_phase_to_looong_signal(&signal, 10_000)
	});

	println!(
		"After 100 iterations, the signal starts with {:?}.",
		&output[..8]
	);
}
