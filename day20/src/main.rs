use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
struct Position {
	index: usize,
	value: i32,
}

impl Position {
	fn parse(index: usize, value: &str) -> Self {
		Self {
			index,
			value: value.parse().unwrap(),
		}
	}
}

fn main() {
	let file = File::open("./day20/input").unwrap();

	let positions: Vec<Position> = BufReader::new(file).lines()
		.enumerate()
		.map(|(i, line)| Position::parse(i, &line.unwrap()))
		.collect();

	let mut result = positions.clone();

	for position in &positions {
		let index = result.iter()
			.position(|pos| pos.index == position.index)
			.unwrap();
		let current = result.remove(index);
		let added = index as i32 + current.value;
		let result_index = added.rem_euclid(result.len() as i32);
		result.insert(result_index as usize, current);
	}

	let start_index = result.iter()
		.position(|pos| pos.value == 0)
		.unwrap();

	// Part 1
	println!("Sum: {}", [1_000, 2_000, 3_000]
		.map(|i| result.iter().cycle().nth(start_index + i).unwrap().value)
		.into_iter()
		.sum::<i32>());
}
