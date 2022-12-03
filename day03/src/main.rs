use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("./day03/input").unwrap();

	let lines: Vec<String> = BufReader::new(file).lines()
		.map(|result| result.unwrap())
		.collect();

	let mut sum1 = 0;
	let mut sum2 = 0;

	for i in 0..lines.len() / 3 {
		for j in 0..3 {
			let line = &lines[i * 3 + j];
			let other: HashSet<char> = line[..line.len() / 2].chars().collect();

			sum1 += line[line.len() / 2..].chars()
				.filter(|item| other.contains(item))
				.collect::<HashSet<char>>()
				.iter()
				.map(|item| get_priority(*item))
				.sum::<u32>();
		}

		let intersection = lines[i * 3 + 1..i * 3 + 3]
			.iter()
			.fold(lines[i * 3].to_string(), |str, line| str.chars()
				.filter(|item| line.contains(*item))
				.collect());

		sum2 += get_priority(intersection.chars().next().unwrap());
	}

	// Part 1
	println!("Priorities: {}", sum1);

	// Part 2
	println!("Badges: {}", sum2);
}

fn get_priority(item: char) -> u32 {
	if item.is_lowercase() {
		item as u32 - 'a' as u32 + 1
	} else {
		item as u32 - 'A' as u32 + 27
	}
}
