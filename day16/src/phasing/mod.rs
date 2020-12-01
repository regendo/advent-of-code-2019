use itertools::Itertools;
use std::convert::TryFrom;

enum Alternate {
	Positive,
	Negative,
}

pub fn apply_pattern(dilation: usize, signal: &[u8], signal_repetitions: usize) -> u8 {
	(signal
		.iter()
		// Our pattern is 0^n+1, 1^n+1, 0^n+1, -1^n+1
		// We're supposed to skip the first 0
		// But we don't care about computing the next n 0s either.
		.skip(dilation)
		.map(|u| i32::try_from(*u).unwrap())
		// Split into chunks of n+1 digits each
		// These chunks align with the repeating pattern of 1, 0, -1, 0, ...
		.chunks(dilation + 1)
		.into_iter()
		// Skip every second chunk, because it is 0
		.step_by(2)
		.fold(
			(Alternate::Positive, 0_i32),
			|(sign, acc), elem| match sign {
				Alternate::Positive => (Alternate::Negative, acc + elem.sum::<i32>()),
				Alternate::Negative => (Alternate::Positive, acc - elem.sum::<i32>()),
			},
		)
		.1
		.abs() % 10) as u8
}

pub fn apply_phase(signal: &[u8]) -> Vec<u8> {
	(0..signal.len())
		.map(|i| apply_pattern(i, signal, 1))
		.collect()
}

pub fn apply_phase_to_looong_signal(signal: &[u8], signal_repetitions: usize) -> Vec<u8> {
	(0..signal.len())
		.map(|i| apply_pattern(i, signal, signal_repetitions))
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_1_single_pattern() {
		let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];

		assert_eq!(4, apply_pattern(0, &signal, 1));
	}

	#[test]
	fn example_1_single_phase() {
		let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
		let expected = vec![4, 8, 2, 2, 6, 1, 5, 8];

		assert_eq!(expected, apply_phase(&signal));
	}

	#[test]
	fn example_1_full() {
		let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
		let expected = vec![0, 1, 0, 2, 9, 4, 9, 8];

		let actual = (0..4).fold(signal, |signal, _| apply_phase(&signal));

		assert_eq!(expected, actual);
	}

	#[test]
	fn example_2() {
		let iterations = 100;

		let test = move |input: &str, expected: &str| {
			assert_eq!(
				crate::str_to_digits(expected),
				(0..iterations).fold(crate::str_to_digits(input), |signal, _| {
					apply_phase(&signal)
				})[..8]
			)
		};

		test("80871224585914546619083218645595", "24176176");
		test("19617804207202209144916044189917", "73745418");
		test("69317163492948606335995924319873", "52432133");
	}

	#[ignore]
	#[test]
	fn example_3() {
		let iterations = 100;
		let digits_in_offset = 7;

		let test = move |source: &str, expected: &str| {
			let signal = crate::str_to_digits(source);
			let expected = crate::str_to_digits(expected);

			let offset: usize = (0..digits_in_offset).fold(0_usize, |acc, index| {
				acc * 10 + usize::try_from(*signal.get(index).unwrap()).unwrap()
			});

			let computed = (0..iterations).fold(signal, |signal, _| {
				apply_phase_to_looong_signal(&*signal, 10_000)
			});
			assert_eq!(expected, computed[offset..offset + 8]);
		};

		test("03036732577212944063491565474664", "84462026");
		test("02935109699940807407585447034323", "78725270");
		test("03081770884921959731165446850517", "53553731");
	}
}
