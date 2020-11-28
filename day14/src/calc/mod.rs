use std::collections::HashMap;

use crate::parse::Reaction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resources<'a> {
	required: HashMap<&'a str, u32>,
	leftover: HashMap<&'a str, u32>,
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
}
