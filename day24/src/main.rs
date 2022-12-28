use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Error};

struct Map {
	tiles: HashMap<Position, Vec<Tile>>,
	width: u32,
	height: u32,
}

struct MapCollection {
	maps: Vec<Map>,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Tile {
	Start,
	End,
	Wall,
	Blizzard(Direction),
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Position {
	x: i32,
	y: i32,
}

impl Map {
	fn new() -> Self {
		Self {
			tiles: HashMap::new(),
			width: 0,
			height: 0,
		}
	}

	fn parse(lines: impl Iterator<Item=Result<String, Error>>) -> Self {
		let mut map = Map::new();

		for (y, result) in lines.enumerate() {
			map.height = max(map.height, y as u32 + 1);
			for (x, char) in result.unwrap().chars().enumerate() {
				map.width = max(map.width, x as u32 + 1);
				if let Some(tile) = Tile::parse(char) {
					map.tiles.insert(Position::new(x as i32, y as i32), vec![tile]);
				}
			}
		}

		map.tiles.insert(Position::new(1, 0), vec![Tile::Start]);
		map.tiles.insert(Position::new(map.width as i32 - 2, map.height as i32 - 1),
			vec![Tile::End]);

		map
	}

	fn step(&self) -> Self {
		let mut map: HashMap<Position, Vec<Tile>> = HashMap::new();

		for (position, tiles) in &self.tiles {
			if let Some(Tile::Wall | Tile::Start | Tile::End) = tiles.first() {
				assert_eq!(tiles.len(), 1);
				map.insert(position.clone(), tiles.clone());
			}
		}

		for (position, tiles) in &self.tiles {
			for tile in tiles {
				if let Tile::Blizzard(dir) = tile {
					let new_pos = position.walk(Some(dir));
					if let Some(existing) = map.get_mut(&new_pos) {
						match existing.first() {
							Some(Tile::Blizzard(_)) => existing.push(tile.clone()),
							Some(Tile::Wall) => {
								let pos = match dir {
									Direction::Up => Position::new(
										position.x,
										self.height as i32 - 2,
									),
									Direction::Right => Position::new(1, position.y),
									Direction::Down => Position::new(position.x, 1),
									Direction::Left => Position::new(
										self.width as i32 - 2,
										position.y,
									),
								};
								if let Some(existing) = map.get_mut(&pos) {
									existing.push(tile.clone());
								} else {
									map.insert(pos, vec![tile.clone()]);
								}
							},
							_ => panic!("Unknown tile"),
						};
					} else {
						map.insert(new_pos, vec![tile.clone()]);
					}
				}
			}
		}

		Self {
			tiles: map,
			width: self.width,
			height: self.height,
		}
	}

	fn find(&self, tile: &Tile) -> Option<Position> {
		self.tiles.iter()
			.find(|(_, tiles)| tiles.contains(tile))
			.map(|(pos, _)| pos.clone())
	}
}

impl MapCollection {
	fn new(map: Map) -> Self {
		Self {
			maps: Vec::from([map]),
		}
	}

	fn get(&mut self, index: usize) -> &Map {
		if index < self.maps.len() {
			&self.maps[index]
		} else {
			for _ in self.maps.len()..=index {
				let map = self.maps.last().unwrap().step();
				self.maps.push(map);
			}
			self.maps.last().unwrap()
		}
	}
}

impl Tile {
	fn parse(ch: char) -> Option<Self> {
		match ch {
			'.' => None,
			'#' => Some(Tile::Wall),
			'<' => Some(Tile::Blizzard(Direction::Left)),
			'>' => Some(Tile::Blizzard(Direction::Right)),
			'^' => Some(Tile::Blizzard(Direction::Up)),
			'v' => Some(Tile::Blizzard(Direction::Down)),
			_ => panic!("Unknown char: {}", ch),
		}
	}
}

impl Position {
	fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	fn walk(&self, dir: Option<&Direction>) -> Self {
		match dir {
			Some(Direction::Up) => Self::new(self.x, self.y - 1),
			Some(Direction::Right) => Self::new(self.x + 1, self.y),
			Some(Direction::Down) => Self::new(self.x, self.y + 1),
			Some(Direction::Left) => Self::new(self.x - 1, self.y),
			None => Self::new(self.x, self.y),
		}
	}
}

fn walk(maps: &mut MapCollection, start: Position) -> usize {
	let mut positions = HashSet::from([start]);
	let mut steps = 0;

	loop {
		let mut options = HashSet::new();
		let map = maps.get(steps);

		for position in positions {
			let directions = [
				None,
				Some(&Direction::Up),
				Some(&Direction::Right),
				Some(&Direction::Down),
				Some(&Direction::Left),
			];
			for direction in directions {
				let next = position.walk(direction);
				if let Some(tiles) = map.tiles.get(&next) {
					if tiles[0] == Tile::End {
						return steps;
					}
				} else if next.y > 0 {
					options.insert(next);
				}
			}
		}

		positions = if options.len() > 0 {
			options
		} else {
			HashSet::from([start])
		};

		steps += 1;
	}
}

fn main() {
	let file = File::open("./day24/input").unwrap();
	let lines = BufReader::new(file).lines();

	let mut maps = MapCollection::new(Map::parse(lines));
	let start = maps.get(0).find(&Tile::Start).unwrap();

	// Part 1
	println!("Steps: {}", walk(&mut maps, start));
}
