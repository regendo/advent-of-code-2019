mod input;

fn criteria_six_digits(num: u32) -> bool {
	num >= 100_000 && num <= 999_999
}

fn criteria_in_range(num: u32) -> bool {
	num >= input::LOWER && num <= input::UPPER
}

fn criteria_two_same(num: u32) -> bool {
	let mut num = num;
	while num > 0 {
		if num % 10 == num / 10 % 10 {
			return true;
		}
		num /= 10;
	}
	false
}

fn criteria_two_same_strictly(num: u32) -> bool {
	let mut digits = [0; 10];
	let mut num = num;
	while num > 0 {
		digits[(num % 10) as usize] += 1;
		num /= 10;
	}
	digits.iter().any(|n| *n == 2)
}

fn criteria_non_descending(num: u32) -> bool {
	let mut num = num;
	while num > 0 {
		if num % 10 < num / 10 % 10 {
			return false;
		}
		num /= 10;
	}
	true
}

pub fn count_valid_options() -> u32 {
	(input::LOWER..=input::UPPER)
		.filter(|n| {
			// We don't _really_ need the first two since we're already iterating over 6-digit in-range numbers.
			criteria_six_digits(*n)
				&& criteria_in_range(*n)
				&& criteria_two_same(*n)
				&& criteria_non_descending(*n)
		})
		.count() as u32
}

pub fn count_valid_options_strictly() -> u32 {
	(input::LOWER..=input::UPPER)
		.filter(|n| {
			// We don't _really_ need the first two since we're already iterating over 6-digit in-range numbers.
			criteria_six_digits(*n)
				&& criteria_in_range(*n)
				&& criteria_two_same_strictly(*n)
				&& criteria_non_descending(*n)
		})
		.count() as u32
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn spec_double_11() {
		let num = 111_111;
		let valid =
			criteria_non_descending(num) && criteria_six_digits(num) && criteria_two_same(num);
		assert_eq!(valid, true);
	}
	#[test]
	fn spec_decreasing() {
		let num = 223_450;
		let valid =
			criteria_non_descending(num) && criteria_six_digits(num) && criteria_two_same(num);
		assert_eq!(valid, false);
	}
	#[test]
	fn spec_no_double() {
		let num = 123_789;
		let valid =
			criteria_non_descending(num) && criteria_six_digits(num) && criteria_two_same(num);
		assert_eq!(valid, false);
	}

	#[test]
	fn spec_all_double() {
		let num = 11_22_33;
		let valid = criteria_non_descending(num)
			&& criteria_six_digits(num)
			&& criteria_two_same_strictly(num);
		assert_eq!(valid, true);
	}

	#[test]
	fn spec_just_triplet() {
		let num = 123_444;
		let valid = criteria_non_descending(num)
			&& criteria_six_digits(num)
			&& criteria_two_same_strictly(num);
		assert_eq!(valid, false);
	}

	#[test]
	fn spec_with_multiplet() {
		let num = 111_122;
		let valid = criteria_non_descending(num)
			&& criteria_six_digits(num)
			&& criteria_two_same_strictly(num);
		assert_eq!(valid, true);
	}
}
