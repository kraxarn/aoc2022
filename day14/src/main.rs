use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
	x: u32,
	y: u32,
}

impl Position {
	fn new(x: u32, y: u32) -> Self {
		Position { x, y }
	}
	fn parse(pos: &str) -> Self {
		let parts: Vec<u32> = pos.split(',')
			.map(|val| val.parse::<u32>().unwrap())
			.collect();
		Self { x: parts[0], y: parts[1] }
	}
	fn until(&self, other: &Self) -> Vec<Position> {
		if self.x == other.x {
			(min(self.y, other.y)..=max(self.y, other.y))
				.map(|y| Position::new(self.x, y))
				.collect()
		} else if self.y == other.y {
			(min(self.x, other.x)..=max(self.x, other.x))
				.map(|x| Position::new(x, self.y))
				.collect()
		} else {
			panic!("Diagonals not supporter")
		}
	}
	fn down(&self) -> Position {
		Self::new(self.x, self.y + 1)
	}
	fn down_left(&self) -> Position {
		Self::new(self.x - 1, self.y + 1)
	}
	fn down_right(&self) -> Position {
		Self::new(self.x + 1, self.y + 1)
	}
	fn move_down(&mut self) {
		self.y += 1;
	}
	fn move_down_left(&mut self) {
		self.move_down();
		self.x -= 1;
	}
	fn move_down_right(&mut self) {
		self.move_down();
		self.x += 1;
	}
}

#[derive(Eq, PartialEq)]
enum Tile {
	Rock,
	Sand,
}

fn find_abyss<'a>(map: impl Iterator<Item=(&'a Position, &'a Tile)>) -> Option<u32> {
	map.map(|(pos, _)| pos.y).max()
}

fn get_sand_position(map: &HashMap<Position, Tile>, limit: u32, floor: bool) -> Option<Position> {
	let mut position = Position::new(500, 0);
	while position.y < limit {
		if !map.contains_key(&position.down()) {
			position.move_down();
		} else if !map.contains_key(&position.down_left()) {
			position.move_down_left();
		} else if !map.contains_key(&position.down_right()) {
			position.move_down_right();
		} else {
			return if position.y == 0 { None } else { Some(position) }
		}
	}
	if floor { Some(position) } else { None }
}

fn get_tile_count<'a>(map: impl Iterator<Item=(&'a Position, &'a Tile)>, tile: &Tile) -> usize {
	map.filter(|(_, map_tile)| map_tile == &tile).count()
}

fn remove_tiles(map: &mut HashMap<Position, Tile>, tile: &Tile) {
	let values: Vec<Position> = map.iter()
		.filter(|(_, map_tile)| map_tile == &tile)
		.map(|(&pos, _)| pos)
		.collect();
	for value in values {
		map.remove(&value);
	}
}

fn simulate(map: &mut HashMap<Position, Tile>, limit: u32, floor: bool) {
	while let Some(position) = get_sand_position(&map, limit, floor) {
		map.insert(position, Tile::Sand);
	}
}

fn main() {
	let file = File::open("./day14/input").unwrap();
	let lines = BufReader::new(file).lines();
	let mut map = HashMap::new();

	for result in lines {
		let line = result.unwrap();
		let mut points = line.split(" -> ")
			.map(|pos| Position::parse(pos))
			.peekable();
		while let Some(a) = points.next() {
			if let Some(b) = points.peek() {
				for position in a.until(&b) {
					map.insert(position, Tile::Rock);
				}
			}
		}
	}

	let abyss = find_abyss(map.iter()).unwrap();
	simulate(&mut map, abyss, false);

	// Part 1
	println!("Abyss sand tiles: {}", get_tile_count(map.iter(), &Tile::Sand));

	remove_tiles(&mut map, &Tile::Sand);
	simulate(&mut map, abyss + 1, true);
	map.insert(Position::new(500, 0), Tile::Sand);

	// Part 2
	println!("Floor sand tiles: {}", get_tile_count(map.iter(), &Tile::Sand));
}
