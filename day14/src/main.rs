use day14::{calc, parse};
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
	let raw_input = fs::read_to_string("input.txt")?;
	let reactions = parse::parse_reactions(&raw_input)?;
	let resources = calc::create_next_chemical(&reactions, Default::default())?;

	println!(
		"Used {} ORE to create 1 FUEL.",
		resources.required.get("ORE").unwrap()
	);

	let mut step = 100_000_u64;
	let mut resources = calc::Resources {
		leftover: vec![("ORE", 1_000000_000000)].into_iter().collect(),
		required: vec![("FUEL", step)].into_iter().collect(),
	};
	loop {
		let _resources = calc::create_next_chemical(&reactions, resources.clone()).unwrap();
		if let Some(ore) = _resources.required.get("ORE") {
			// We went too far
			*resources.required.get_mut("FUEL").unwrap() -= step;
			if step <= 1 {
				break;
			} else {
				step /= 10;
				*resources.required.get_mut("FUEL").unwrap() += step;
			}
		} else {
			*resources.required.get_mut("FUEL").unwrap() += step;
		}
	}
	println!(
		"With 1 TRILLION ORE, we can create {} FUEL.",
		resources.required.get("FUEL").unwrap()
	);

	Ok(())
}
