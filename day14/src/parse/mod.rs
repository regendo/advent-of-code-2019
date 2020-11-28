use std::{collections::HashMap, error::Error};

fn try_parse_chemical(value: &str) -> Result<(&str, u32), Box<dyn Error>> {
	let mut split = value.trim().split_whitespace();
	if let (Some(amount), Some(name), None) = (split.next(), split.next(), split.next()) {
		Ok((name, u32::from_str_radix(amount, 10)?))
	} else {
		Err(format!("Unable to convert {} into a Chemical.", value).into())
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reaction<'a> {
	pub input: HashMap<&'a str, u32>,
	pub output: (&'a str, u32),
}

pub fn parse_reactions(source: &str) -> Result<Vec<Reaction>, Box<dyn Error>> {
	Ok(source
		.lines()
		.filter(|line| !line.trim().is_empty())
		.filter_map(|line| {
			let mut split = line.split("=>");
			if let (Some(left), Some(right), None) = (split.next(), split.next(), split.next()) {
				Some(Reaction {
					input: left
						.split(',')
						.map(try_parse_chemical)
						.collect::<Result<HashMap<&str, u32>, _>>()
						.ok()?,
					output: try_parse_chemical(right).ok()?,
				})
			} else {
				None
			}
		})
		.collect())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_constructs_chemicals() {
		assert_eq!(try_parse_chemical("10 ORE").unwrap(), ("ORE", 10));
	}

	#[test]
	fn it_constructs_the_example_reactions() {
		let raw = r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
		let constructed = parse_reactions(raw).unwrap();
		assert_eq!(constructed.len(), 6);
		assert_eq!(
			constructed,
			vec![
				Reaction {
					input: vec![("ORE", 10)].into_iter().collect(),
					output: ("A", 10)
				},
				Reaction {
					input: vec![("ORE", 1)].into_iter().collect(),
					output: ("B", 1)
				},
				Reaction {
					input: vec![("A", 7), ("B", 1)].into_iter().collect(),
					output: ("C", 1)
				},
				Reaction {
					input: vec![("A", 7), ("C", 1)].into_iter().collect(),
					output: ("D", 1)
				},
				Reaction {
					input: vec![("A", 7), ("D", 1)].into_iter().collect(),
					output: ("E", 1)
				},
				Reaction {
					input: vec![("A", 7), ("E", 1)].into_iter().collect(),
					output: ("FUEL", 1)
				},
			]
		);
	}
}
