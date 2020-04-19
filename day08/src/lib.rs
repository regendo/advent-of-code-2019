use std::fs;

fn read_input(path: &str) -> Vec<u8> {
	fs::read_to_string(path)
		.unwrap()
		.trim()
		.split("")
		.filter(|s| !s.is_empty())
		.map(|s| s.parse::<u8>().unwrap())
		.collect()
}
