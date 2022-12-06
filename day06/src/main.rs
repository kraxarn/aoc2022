use std::collections::HashSet;
use std::fs;

fn main() {
	let line = fs::read_to_string("./day06/input").unwrap();

	let mut start_index = 0_usize;
	let mut message_index = 0_usize;

	for i in 0..line.len() - 13 {
		if start_index == 0 && has_unique(&line[i..], 4) {
			start_index = i + 4;
		}
		if message_index == 0 && has_unique(&line[i..], 14) {
			message_index = i + 14;
		}
		if message_index > 0 && start_index > 0 {
			break;
		}
	}

	// Part 1
	println!("Data index: {}", start_index);

	// Part 2
	println!("Message index: {}", message_index);
}

fn has_unique(str: &str, count: usize) -> bool {
	str[..count].chars().collect::<HashSet<char>>().len() == count
}