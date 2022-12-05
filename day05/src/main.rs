use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("./day05/input").unwrap();
	let lines = BufReader::new(file).lines();

	let mut crates1: Vec<Vec<char>> = (0..9)
		.map(|_| Vec::new())
		.collect();

	let mut crates2 = Vec::new();

	for result in lines {
		let line = result.unwrap();

		if line.starts_with('[') {
			let chars: Vec<char> = line.chars().collect();
			for i in 0..crates1.len() {
				let content = chars[1 + i * 4];
				if content.is_alphabetic() {
					crates1[i].insert(0, content);
				}
			}
			continue;
		}

		if crates2.is_empty() {
			crates1.clone_into(&mut crates2);
		}

		if !line.starts_with("move") {
			continue;
		}

		let parts: Vec<&str> = line.split(' ').collect();

		let crate_count = parts[1].parse::<usize>().unwrap();
		let from_pos = parts[3].parse::<usize>().unwrap();
		let to_pos = parts[5].parse::<usize>().unwrap();

		for _ in 0..crate_count {
			let item = crates1[from_pos - 1].pop().unwrap();
			crates1[to_pos - 1].push(item);
		}

		let items: Vec<_> = (0..crate_count)
			.map(|_| crates2[from_pos - 1].pop().unwrap())
			.collect();

		for item in items.iter().rev() {
			crates2[to_pos - 1].push(*item);
		}
	}

	// Part 1
	println!("Top-most using 9000: {}", &get_top_most_crates(&crates1));

	// Part 2
	println!("Top-most using 9001: {}", &get_top_most_crates(&crates2));
}

fn get_top_most_crates(crates: &[Vec<char>]) -> String {
	crates.iter()
		.map(|stack| stack.last().unwrap())
		.collect()
}