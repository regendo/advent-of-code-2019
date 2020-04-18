use day05;
use day07::find_optimal_phases;

fn main() {
	let program = day05::load_program("input.txt").expect("Failed loading the program!");
	if let Some((phases, output)) = find_optimal_phases(&program) {
		println!(
			"Max thruster signal `{}` (from phase setting sequence {:?})",
			output, phases
		);
	}
}
