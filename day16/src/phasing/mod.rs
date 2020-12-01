use itertools::Itertools;
use num::integer as num;
use std::convert::TryFrom;

enum Alternate {
	Positive,
	Negative,
}

fn alternating_sum((sign, acc): (Alternate, i32), num: i32) -> (Alternate, i32) {
	match sign {
		Alternate::Positive => (Alternate::Negative, acc + num),
		Alternate::Negative => (Alternate::Positive, acc - num),
	}
}

pub fn apply_pattern(dilation: usize, signal: &[u8], signal_repetitions: usize) -> u8 {
	let phase_len = 1 + dilation;
	let total_phase_len = 4 * phase_len;
	let total_signal_len = signal_repetitions * signal.len();
	let matches_repeat_after = num::lcm(total_phase_len, signal.len());
	let matches_in_repetition = matches_repeat_after / total_phase_len;
	let factor = i32::try_from(total_signal_len / matches_repeat_after).unwrap();

	let optimize = total_signal_len > matches_repeat_after + dilation;

	let len_taken = if optimize {
		// skipped offset + main repeating batch + leftover at the back
		dilation + matches_repeat_after + (total_signal_len % matches_repeat_after)
	} else {
		total_signal_len
	};

	(signal
		.iter()
		.cycle()
		.take(len_taken)
		// Our pattern is 0^n+1, 1^n+1, 0^n+1, -1^n+1
		// We're supposed to skip the first 0
		// But we don't care about computing the next n 0s either.
		.skip(dilation)
		.map(|u| i32::try_from(*u).unwrap())
		// These chunks align with the repeating pattern of 1, 0, -1, 0, ...
		.chunks(phase_len)
		.into_iter()
		// Skip every second chunk, because it is 0
		.step_by(2)
		.enumerate()
		.map(|(idx, chunk)| {
			// times two because we've got two chunks per repitition
			if optimize && idx < 2 * matches_in_repetition {
				chunk.sum::<i32>() * factor
			} else {
				chunk.sum()
			}
		})
		.fold((Alternate::Positive, 0), alternating_sum)
		.1
		.abs() % 10) as u8
}

pub fn apply_phase(signal: &[u8]) -> Vec<u8> {
	(0..signal.len())
		.map(|i| apply_pattern(i, signal, 1))
		.collect()
}

pub fn apply_phase_to_looong_signal(signal: &[u8], signal_repetitions: usize) -> Vec<u8> {
	// TODO move the optimization out here
	// Oh lord
	// This computes a ~320_000 digit signal on the first run, and optimizes it alright
	// It then returns that long-ass signal
	// And takes a request for a 3_200_000_000 digit signal
	// and so on
	// This is completely fucked, and my optimization above does jack shit.
	(0..signal.len() * signal_repetitions)
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
