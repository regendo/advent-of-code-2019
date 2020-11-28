use std::{convert::TryFrom, error::Error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chemical {
	name: String,
	amount: u8,
}

impl TryFrom<&str> for Chemical {
	type Error = Box<dyn Error>;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let mut split = value.trim().split_whitespace();
		if let (Some(amount), Some(name), None) = (split.next(), split.next(), split.next()) {
			Ok(Self {
				name: name.to_owned(),
				amount: u8::from_str_radix(amount, 10)?,
			})
		} else {
			Err(format!("Unable to convert {} into a Chemical.", value))?
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Reaction {
	input: Vec<Chemical>,
	output: Chemical,
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
						.map(Chemical::try_from)
						.collect::<Result<Vec<Chemical>, _>>()
						.ok()?,
					output: Chemical::try_from(right).ok()?,
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
		assert_eq!(
			Chemical::try_from("10 ORE").unwrap(),
			Chemical {
				name: "ORE".to_owned(),
				amount: 10
			}
		);
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
					input: vec![Chemical {
						name: "ORE".to_owned(),
						amount: 10
					}],
					output: Chemical {
						name: "A".to_owned(),
						amount: 10
					}
				},
				Reaction {
					input: vec![Chemical {
						name: "ORE".to_owned(),
						amount: 1
					}],
					output: Chemical {
						name: "B".to_owned(),
						amount: 1
					}
				},
				Reaction {
					input: vec![
						Chemical {
							name: "A".to_owned(),
							amount: 7
						},
						Chemical {
							name: "B".to_owned(),
							amount: 1
						}
					],
					output: Chemical {
						name: "C".to_owned(),
						amount: 1
					}
				},
				Reaction {
					input: vec![
						Chemical {
							name: "A".to_owned(),
							amount: 7
						},
						Chemical {
							name: "C".to_owned(),
							amount: 1
						}
					],
					output: Chemical {
						name: "D".to_owned(),
						amount: 1
					}
				},
				Reaction {
					input: vec![
						Chemical {
							name: "A".to_owned(),
							amount: 7
						},
						Chemical {
							name: "D".to_owned(),
							amount: 1
						}
					],
					output: Chemical {
						name: "E".to_owned(),
						amount: 1
					}
				},
				Reaction {
					input: vec![
						Chemical {
							name: "A".to_owned(),
							amount: 7
						},
						Chemical {
							name: "E".to_owned(),
							amount: 1
						}
					],
					output: Chemical {
						name: "FUEL".to_owned(),
						amount: 1
					}
				},
			]
		);
	}
}
