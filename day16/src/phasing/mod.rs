use crate::stutter;
use std::convert::TryFrom;

pub fn apply_phase(number: u8, signal: &[u8], pattern: &[i32]) -> Vec<u32> {
    let mut iter = stutter::Stutter::new(pattern.iter().cycle(), number).zip(signal.iter().cycle());
    iter.skip(1);

    (0..signal.len())
        .map(|_| {
            iter.take(signal.len())
                .map(|(a, b)| a * (*b as i32))
                .sum::<i32>()
        })
        .map(|sum| (sum.abs() % 10) as u32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let phase = vec![0, 1, 0, -1];
        let expected = vec![4, 8, 2, 2, 6, 1, 5, 8];

        assert_eq!(expected, apply_phase(0, &signal, &phase));
    }
}
