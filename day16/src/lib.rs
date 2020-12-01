mod phasing;
pub mod stutter;

fn solve_1() {
    let signal: Vec<u32> = include_str!("input.txt")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let base_pattern = vec![0, 1, 0, -1];
}
