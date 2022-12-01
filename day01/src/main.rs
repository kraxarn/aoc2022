use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("./day01/input").unwrap();
	let lines = BufReader::new(file).lines();

	let mut current = 0_u32;
	let mut results = Vec::new();

	for result in lines {
		let line = result.unwrap();

		if let Ok(value) = line.parse::<u32>() {
			current += value;
		} else {
			results.push(current);
			current = 0_u32;
		}
	}

	results.sort_by(|a, b| b.cmp(a));

	// Part 1
	println!("Most calories: {}", results.first().unwrap());

	// Part 2
	println!("Top 3 most calories: {}", results[0..3].iter().sum::<u32>());
}
