use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash)]
struct Position {
	x: u32,
	y: u32,
	z: u32,
}

impl Position {
	fn new(x: u32, y: u32, z: u32) -> Self {
		Self { x, y, z }
	}
}

impl FromStr for Position {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<u32> = s.split(',')
			.map(|part| part.parse::<u32>().unwrap())
			.collect();
		Ok(Position::new(parts[0], parts[1], parts[2]))
	}
}

fn get_open_sides(positions: &HashSet<Position>) -> u32 {
	let mut result = 0;

	for pos in positions {
		if pos.x == 0 || !positions.contains(&Position::new(pos.x - 1, pos.y, pos.z)) {
			result += 1;
		}
		if !positions.contains(&Position::new(pos.x + 1, pos.y, pos.z)) {
			result += 1;
		}
		if pos.y == 0 || !positions.contains(&Position::new(pos.x, pos.y - 1, pos.z)) {
			result += 1;
		}
		if !positions.contains(&Position::new(pos.x, pos.y + 1, pos.z)) {
			result += 1;
		}
		if pos.z == 0 || !positions.contains(&Position::new(pos.x, pos.y, pos.z - 1)) {
			result += 1;
		}
		if !positions.contains(&Position::new(pos.x, pos.y, pos.z + 1)) {
			result += 1;
		}
	}

	result
}

fn main() {
	let file = File::open("./day18/input").unwrap();
	let positions: HashSet<Position> = BufReader::new(file).lines()
		.map(|line| line.unwrap().parse::<Position>().unwrap())
		.collect();

	// Part 1
	println!("Open sides: {}", get_open_sides(&positions));
}
