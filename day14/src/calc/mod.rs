use std::collections::HashMap;

use crate::parse::Reaction;

#[derive(Debug, Clone)]
struct Resources<'a> {
	required: HashMap<&'a str, u32>,
	leftover: HashMap<&'a str, u32>,
}

pub fn what_creates<'a, 'b>(haystack: &'a [Reaction<'b>], needle: &str) -> Vec<&'a Reaction<'b>> {
	haystack.iter().filter(|r| r.output.0 == needle).collect()
}

/// Compute the resources gained and used up in one reaction.
/// Does not modify the input resources.
fn react<'a>(reaction: &Reaction<'a>, times: u32, resources: Resources<'a>) -> Resources<'a> {
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
				*req = 0;
				amount_created.saturating_sub(*req)
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
				*stash = 0;
				amount_used.saturating_sub(*stash)
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
