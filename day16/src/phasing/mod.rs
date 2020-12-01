use crate::stutter;
use std::convert::TryFrom;

pub fn apply_pattern(number: u32, signal: &[u8], pattern: &[i8]) -> u8 {
	(stutter::Stutter::new(pattern.iter().cycle(), number)
		.skip(1)
		.zip(signal.iter().cycle())
		.take(signal.len())
		.map(|(a, b)| i32::try_from(a * i8::try_from(*b).unwrap()).unwrap())
		.sum::<i32>()
		.abs() % 10) as u8
}

pub fn apply_phase(signal: &[u8], pattern: &[i8]) -> Vec<u8> {
	(0..signal.len())
		.map(|i| apply_pattern(u32::try_from(i).unwrap(), signal, pattern))
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_1_single_pattern() {
		let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
		let phase = vec![0, 1, 0, -1];

		assert_eq!(4, apply_pattern(0, &signal, &phase));
	}

	#[test]
	fn example_1_single_phase() {
		let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
		let phase = vec![0, 1, 0, -1];
		let expected = vec![4, 8, 2, 2, 6, 1, 5, 8];

		assert_eq!(expected, apply_phase(&signal, &phase));
	}

	#[test]
	fn example_1_full() {
		let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
		let phase = vec![0, 1, 0, -1];
		let expected = vec![0, 1, 0, 2, 9, 4, 9, 8];

		let actual = (0..4).fold(signal, |signal, _| apply_phase(&signal, &phase));

		assert_eq!(expected, actual);
	}

	#[test]
	fn example_2() {
		let phase = vec![0, 1, 0, -1];
		let iterations = 100;

		let test = move |input: &str, expected: &str| {
			assert_eq!(
				crate::str_to_digits(expected),
				(0..iterations).fold(crate::str_to_digits(input), |signal, _| apply_phase(
					&signal, &phase
				))[..8]
			)
		};

		test("80871224585914546619083218645595", "24176176");
		test("19617804207202209144916044189917", "73745418");
		test("69317163492948606335995924319873", "52432133");
	}
}
