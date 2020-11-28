use std::{collections::HashMap, error::Error};

use crate::parse::Reaction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resources<'a> {
	pub required: HashMap<&'a str, u32>,
	leftover: HashMap<&'a str, u32>,
}

impl Default for Resources<'static> {
	fn default() -> Self {
		Self {
			required: vec![("FUEL", 1)].into_iter().collect(),
			leftover: Default::default(),
		}
	}
}

pub fn what_creates<'a, 'b>(haystack: &'a [Reaction<'b>], needle: &str) -> Vec<&'a Reaction<'b>> {
	haystack.iter().filter(|r| r.output.0 == needle).collect()
}

/// Compute the resources gained and used up in one reaction.
/// Does not modify the input resources.
pub fn react<'a>(reaction: &Reaction<'a>, times: u32, resources: Resources<'a>) -> Resources<'a> {
	let mut resources = resources.clone();

	{
		// Add the newly created chemical
		let chemical_created = reaction.output.0;
		let amount_created = reaction.output.1 * times;
		let amount_left_over = if let Some(req) = resources.required.get_mut(chemical_created) {
			if *req >= amount_created {
				*req = req.saturating_sub(amount_created);
				0
			} else {
				let tmp = amount_created.saturating_sub(*req);
				*req = 0;
				tmp
			}
		} else {
			amount_created
		};
		if amount_left_over > 0 {
			*resources.leftover.entry(chemical_created).or_insert(0) += amount_left_over;
		}
	}

	for (chemical, recipe_amount) in reaction.input.iter() {
		// Remove the used up chemicals
		let amount_used = recipe_amount * times;
		let amount_owed = if let Some(stash) = resources.leftover.get_mut(chemical) {
			if *stash >= amount_used {
				*stash = stash.saturating_sub(amount_used);
				0
			} else {
				let tmp = amount_used.saturating_sub(*stash);
				*stash = 0;
				tmp
			}
		} else {
			amount_used
		};
		if amount_owed > 0 {
			*resources.required.entry(chemical).or_insert(0) += amount_owed;
		}
	}

	resources
}

fn is_wanted<'a>(resource: &'a (&&str, &u32)) -> bool {
	*resource.0 != "ORE" && *resource.1 > 0
}

pub fn create_next_chemical<'a>(
	reactions_available: &[Reaction<'a>],
	resources: Resources<'a>,
) -> Result<Resources<'a>, Box<dyn Error>> {
	let (chemical, wanted_amount) = resources
		.required
		.iter()
		.find(is_wanted)
		.expect("No more chemicals required!");
	let choices = what_creates(reactions_available, chemical);
	if choices.is_empty() {
		return Err(format!("No reaction to create {}!", chemical).into());
	}
	// TODO try other options, find the optimal one
	let reaction = choices.get(0).expect("We checked this earlier.");
	let amount_per_reaction = reaction.output.1;
	let times = if wanted_amount % amount_per_reaction > 0 {
		wanted_amount / amount_per_reaction + 1
	} else {
		wanted_amount / amount_per_reaction
	};

	let resources = react(reaction, times, resources);
	if resources.required.iter().any(|r| is_wanted(&r)) {
		create_next_chemical(reactions_available, resources)
	} else {
		Ok(resources)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_computes_a_reaction() {
		let reaction = Reaction {
			input: vec![("AB", 2), ("BC", 3), ("CA", 4)].into_iter().collect(),
			output: ("FUEL", 1),
		};
		let input_resources = Resources {
			leftover: Default::default(),
			required: vec![("FUEL", 1)].into_iter().collect(),
		};

		let expected = Resources {
			leftover: Default::default(),
			// Our function doesn't remove entries with 0 amount, but that's OK.
			required: vec![("AB", 2), ("BC", 3), ("CA", 4), ("FUEL", 0)]
				.into_iter()
				.collect(),
		};
		assert_eq!(expected, react(&reaction, 1, input_resources));
	}

	#[test]
	fn it_computes_multiples_and_uses_leftovers() {
		let reaction = Reaction {
			input: vec![("AB", 2), ("BC", 3), ("CA", 4)].into_iter().collect(),
			output: ("FUEL", 1),
		};
		let input_resources = Resources {
			leftover: vec![("BC", 8), ("ORE", 9)].into_iter().collect(),
			required: vec![("FUEL", 1)].into_iter().collect(),
		};

		let expected = Resources {
			leftover: vec![("FUEL", 4), ("BC", 0), ("ORE", 9)]
				.into_iter()
				.collect(),
			// Our function doesn't remove entries with 0 amount, but that's OK.
			required: vec![("AB", 10), ("BC", 7), ("CA", 20), ("FUEL", 0)]
				.into_iter()
				.collect(),
		};
		assert_eq!(expected, react(&reaction, 5, input_resources));
	}

	#[test]
	fn it_creates_the_fuel() {
		let reactions = crate::parse::parse_reactions(
			r"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
		)
		.unwrap();
		let input = Resources {
			required: vec![("FUEL", 1)].into_iter().collect(),
			leftover: Default::default(),
		};

		let actual = create_next_chemical(&reactions, input).unwrap();
		assert_eq!(Some(&165), actual.required.get("ORE"));
	}
}
