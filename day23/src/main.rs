use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

enum Direction {
	North,
	South,
	West,
	East,
}

struct Map {
	elves: HashSet<Position>,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
	x: i32,
	y: i32,
}

impl Map {
	fn parse(lines: impl Iterator<Item=Result<String, Error>>) -> Self {
		let mut elves = HashSet::new();

		for (y, result) in lines.enumerate() {
			for (x, char) in result.unwrap().chars().enumerate() {
				if char == '#' {
					elves.insert(Position::new(x as i32, y as i32));
				}
			}
		}

		Self { elves }
	}

	fn top_left(&self) -> Option<Position> {
		let x = self.elves.iter()
			.min_by_key(|pos| pos.x)
			.map(|pos| pos.x);

		let y = self.elves.iter()
			.min_by_key(|pos| pos.y)
			.map(|pos| pos.y);

		match (x, y) {
			(Some(x), Some(y)) => Some(Position::new(x, y)),
			_ => None,
		}
	}

	fn bottom_right(&self) -> Option<Position> {
		let x = self.elves.iter()
			.max_by_key(|pos| pos.x)
			.map(|pos| pos.x);

		let y = self.elves.iter()
			.max_by_key(|pos| pos.y)
			.map(|pos| pos.y);

		match (x, y) {
			(Some(x), Some(y)) => Some(Position::new(x, y)),
			_ => None,
		}
	}

	fn empty_tile_count(&self) -> u32 {
		let mut result = 0;

		let min = self.top_left().unwrap();
		let max = self.bottom_right().unwrap();

		for y in min.y..=max.y {
			for x in min.x..=max.x {
				if !self.elves.contains(&Position::new(x, y)) {
					result += 1;
				}
			}
		}
		result
	}
}

impl Position {
	fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	fn walk(&self, dir: &Direction) -> Self {
		match dir {
			Direction::North => Self::new(self.x, self.y - 1),
			Direction::South => Self::new(self.x, self.y + 1),
			Direction::West => Self::new(self.x - 1, self.y),
			Direction::East => Self::new(self.x + 1, self.y),
		}
	}

	fn adjacent(&self, dir: &Direction) -> Vec<Self> {
		match dir {
			Direction::North | Direction::South => (-1..=1)
				.map(|x| Position::new(self.x + x, self.y))
				.collect(),
			Direction::West | Direction::East => (-1..=1)
				.map(|y| Position::new(self.x, self.y + y))
				.collect()
		}
	}

	fn proposed(&self, map: &Map, n: usize) -> Option<Self> {
		let directions = [
			Direction::North,
			Direction::South,
			Direction::West,
			Direction::East,
		];

		let options: Vec<&Direction> = directions.iter()
			.cycle()
			.skip(n)
			.take(directions.len())
			.filter(|dir| self.walk(dir).adjacent(dir).iter()
				.all(|pos| !map.elves.contains(pos)))
			.collect();

		if options.len() == directions.len() {
			None
		} else {
			options.first()
				.map(|dir| self.walk(dir))
		}
	}
}

fn simulate(map: &mut Map, rounds: usize) {
	for n in 0..rounds {
		let mut elves: Vec<(Position, Option<Position>)> = map.elves.iter()
			.map(|pos| (pos.clone(), pos.proposed(map, n)))
			.collect();

		for i in 0..elves.len() {
			if let Some(new_pos) = elves[i].1 {
				let same_pos: Vec<usize> = elves.iter().enumerate()
					.filter(|(_, (_, pos))| pos.is_some() && pos.unwrap() == new_pos)
					.map(|(i, _)| i)
					.collect();
				if same_pos.len() > 1 {
					for i in same_pos {
						elves[i].1 = None;
					}
				}
			}
		}

		for (old_pos, new_pos) in elves {
			if let Some(new_pos) = new_pos {
				map.elves.remove(&old_pos);
				assert!(map.elves.insert(new_pos));
			}
		}
	}
}

fn main() {
	let file = File::open("./day23/input").unwrap();
	let lines = BufReader::new(file).lines();

	// Part 1
	let mut map = Map::parse(lines);
	simulate(&mut map, 10);
	println!("Empty ground tiles: {}", map.empty_tile_count());
}
