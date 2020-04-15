use day03::*;

fn main() {
	let (lineA, lineB) = read_directions("input.txt").unwrap();

	println!("First line's directions:\n{:?}", lineA);
	println!();
	println!("Second line's directions:\n{:?}", lineB);
}
