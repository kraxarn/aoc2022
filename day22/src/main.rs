use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Tiles = HashMap<Position, TileType>;

#[derive(Eq, PartialEq)]
enum TileType {
	Open,
	Wall,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
	x: i32,
	y: i32,
}

enum Instruction {
	Left,
	Right,
	Forward(usize),
}

enum Direction {
	Right = 0,
	Down = 1,
	Left = 2,
	Up = 3,
}

impl TileType {
	fn parse(ch: char) -> Option<Self> {
		match ch {
			'.' => Some(TileType::Open),
			'#' => Some(TileType::Wall),
			_ => None,
		}
	}
}

impl Position {
	fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}
	fn walk(&self, dir: &Direction) -> Self {
		match dir {
			Direction::Up => Self::new(self.x, self.y - 1),
			Direction::Right => Self::new(self.x + 1, self.y),
			Direction::Down => Self::new(self.x, self.y + 1),
			Direction::Left => Self::new(self.x - 1, self.y),
		}
	}
}

impl Direction {
	fn left(&self) -> Self {
		match self {
			Direction::Right => Direction::Up,
			Direction::Down => Direction::Right,
			Direction::Left => Direction::Down,
			Direction::Up => Direction::Left,
		}
	}
	fn right(&self) -> Self {
		match self {
			Direction::Right => Direction::Down,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
			Direction::Up => Direction::Right,
		}
	}
}

fn parse_tiles(line: &str) -> Vec<(usize, TileType)> {
	line.chars().enumerate()
		.map(|(i, char)| (i, TileType::parse(char)))
		.filter(|(_, tile)| tile.is_some())
		.map(|(i, tile)| (i, tile.unwrap()))
		.collect()
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
	if line.chars().all(|char| !char.is_ascii_uppercase() && !char.is_ascii_digit()) {
		return vec![];
	}

	let mut chars = line.chars().peekable();
	let mut instructions = Vec::new();

	while let Some(current) = chars.next() {
		let instruction = match current {
			'L' => Instruction::Left,
			'R' => Instruction::Right,
			c if c.is_ascii_digit() => {
				let mut result = String::from(c);
				while let Some(next) = chars.peek() {
					if next.is_ascii_digit() {
						result.push(chars.next().unwrap());
					} else {
						break;
					}
				}
				Instruction::Forward(result.parse().unwrap())
			},
			_ => continue,
		};
		instructions.push(instruction);
	}

	instructions
}

fn get_start_position(tiles: &Tiles) -> Position {
	tiles.iter()
		.filter(|(pos, tile)| pos.y == 0 && tile == &&TileType::Open)
		.map(|(&pos, _)| pos)
		.min_by_key(|pos| pos.x)
		.unwrap()
}

fn walk(tiles: &Tiles, position: &Position, direction: &Direction) -> Option<Position> {
	let new_pos = position.walk(direction);
	if let Some(tile) = tiles.get(&new_pos) {
		match tile {
			TileType::Open => Some(new_pos),
			TileType::Wall => None,
		}
	} else {
		let pos = match direction {
			Direction::Right => tiles.keys()
				.filter(|pos| pos.y == position.y)
				.min_by_key(|pos| pos.x),

			Direction::Down => tiles.keys()
				.filter(|pos| pos.x == position.x)
				.min_by_key(|pos| pos.y),

			Direction::Left => tiles.keys()
				.filter(|pos| pos.y == position.y)
				.max_by_key(|pos| pos.x),

			Direction::Up => tiles.keys()
				.filter(|pos| pos.x == position.x)
				.max_by_key(|pos| pos.y),
		};
		if pos.is_some() && tiles[pos.unwrap()] == TileType::Open {
			pos.cloned()
		} else {
			None
		}
	}
}

fn simulate(tiles: &Tiles, instructions: &[Instruction]) -> u32 {
	let mut direction = Direction::Right;
	let mut position = get_start_position(tiles);

	for instruction in instructions {
		match instruction {
			Instruction::Left => direction = direction.left(),
			Instruction::Right => direction = direction.right(),
			Instruction::Forward(steps) => {
				for _ in 0..*steps {
					if let Some(pos) = walk(tiles, &position, &direction) {
						position = pos;
					} else {
						break;
					}
				}
			}
		}
	}

	1_000 * (position.y as u32 + 1)
		+ 4 * (position.x as u32 + 1)
		+ direction as u32
}

fn main() {
	let file = File::open("./day22/input").unwrap();
	let lines = BufReader::new(file).lines();

	let mut tiles = Tiles::new();
	let mut instructions = Vec::new();

	for (y, result) in lines.enumerate() {
		let line = result.unwrap();
		for (x, tile) in parse_tiles(&line) {
			tiles.insert(Position::new(x as i32, y as i32), tile);
		}
		for instruction in parse_instructions(&line) {
			instructions.push(instruction);
		}
	}

	// Part 1
	println!("Password: {}", simulate(&tiles, &instructions[..]));
}
