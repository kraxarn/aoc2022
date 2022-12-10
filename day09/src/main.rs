use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl FromStr for Direction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"U" => Ok(Direction::Up),
			"D" => Ok(Direction::Down),
			"L" => Ok(Direction::Left),
			"R" => Ok(Direction::Right),
			_ => Err(()),
		}
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
	x: i32,
	y: i32,
}

impl Position {
	fn new() -> Self {
		Self { x: 0, y: 0 }
	}

	fn adjacent_area(&self) -> Vec<Self> {
		let mut result = Vec::new();
		for y in self.y - 1..=self.y + 1 {
			for x in self.x - 1..=self.x + 1 {
				result.push(Position { x, y });
			}
		}
		result
	}

	fn is_adjacent_to(&self, other: &Self) -> bool {
		self.adjacent_area().contains(other)
	}

	fn in_direction(&self, direction: &Direction) -> Self {
		match direction {
			Direction::Up => Self { x: self.x, y: self.y - 1 },
			Direction::Down => Self { x: self.x, y: self.y + 1 },
			Direction::Left => Self { x: self.x - 1, y: self.y },
			Direction::Right => Self { x: self.x + 1, y: self.y },
		}
	}

	fn towards(&self, other: &Self) -> Option<Self> {
		if self.x != other.x && self.y != other.y {
			let self_area = self.adjacent_area();
			let positions: Vec<_> = other.adjacent_area()
				.into_iter()
				.filter(|pos| self_area.contains(&pos))
				.collect();
			if positions.len() == 1 {
				Some(positions[0])
			} else {
				Some(positions.into_iter()
					.find(|pos| pos.x == other.x || pos.y == other.y)
					.unwrap())
			}
		} else if self.x == other.x {
			if self.y < other.y {
				Some(self.in_direction(&Direction::Down))
			} else if other.y < self.y {
				Some(self.in_direction(&Direction::Up))
			} else {
				None
			}
		} else if self.y == other.y {
			if self.x < other.x {
				Some(self.in_direction(&Direction::Right))
			} else if other.x < self.x {
				Some(self.in_direction(&Direction::Left))
			} else {
				None
			}
		} else {
			None
		}
	}
}

fn main() {
	let file = File::open("./day09/input").unwrap();
	let lines = BufReader::new(file).lines();

	let mut rope = [Position::new(); 10];
	let mut positions: Vec<_> = rope.iter()
		.map(|&pos| HashSet::from([pos])).collect();

	for result in lines {
		let line = result.unwrap();
		let mut parts = line.split(' ');

		let direction = parts.next().unwrap().parse::<Direction>().unwrap();
		let steps = parts.next().unwrap().parse::<usize>().unwrap();

		for _ in 0..steps {
			rope[0] = rope[0].in_direction(&direction);
			for i in 1..rope.len() {
				if !rope[i - 1].is_adjacent_to(&rope[i]) {
					if let Some(position) = rope[i].towards(&rope[i - 1]) {
						rope[i] = position;
						positions[i].insert(position);
					}
				}
			}
		}
	}

	// Part 1
	println!("Tail positions: {}", positions[1].len());

	// Part 2
	println!("End positions: {}", positions[positions.len() - 1].len());
}
