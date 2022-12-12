use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq)]
struct Point {
	x: usize,
	y: usize,
	steps: u32,
}

impl Point {
	fn new(x: usize, y: usize, steps: u32) -> Self {
		Self { x, y, steps }
	}
}

impl Hash for Point {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.x.hash(state);
		self.y.hash(state);
	}
}

#[derive(Eq, PartialEq)]
enum TileType {
	Start,
	End,
	Tile,
}

struct Tile {
	tile_type: TileType,
	level: u8,
}

impl Tile {
	fn new(id: char) -> Self {
		let (tile_type, level) = match id {
			'S' => (TileType::Start, 'a'),
			'E' => (TileType::End, 'z'),
			level => (TileType::Tile, level),
		};
		Self {
			tile_type,
			level: level as u8,
		}
	}
	fn can_go(&self, from: &Tile, dir: &Direction) -> bool {
		match dir {
			Direction::Uphill => self.level as i32 - from.level as i32 <= 1,
			Direction::Downhill => from.level as i32 - self.level as i32 <= 1,
		}
	}
}

enum Direction {
	Uphill,
	Downhill,
}

fn find(map: &Vec<Vec<Tile>>, tile_type: TileType) -> Point {
	for y in 0..map.len() {
		for x in 0..map[y].len() {
			if map[y][x].tile_type == tile_type {
				return Point::new(x, y, 0);
			}
		}
	}
	panic!("Tile not found");
}

fn walk(
	map: &Vec<Vec<Tile>>, start: Point, dir: &Direction, is_target: fn(&Tile) -> bool
) -> Option<u32> {
	let mut path: Vec<Point> = Vec::from([start]);
	let mut trail: HashSet<Point> = HashSet::new();

	while let Some(point) = path.pop() {
		if trail.contains(&point) {
			continue;
		}
		let x = point.x;
		let y = point.y;
		if is_target(&map[y][x]) {
			return Some(point.steps);
		}
		if x > 0 && map[y][x - 1].can_go(&map[y][x], dir) {
			path.insert(0, Point::new(x - 1, y, point.steps + 1));
		}
		if x < map[y].len() - 1 && map[y][x + 1].can_go(&map[y][x], dir) {
			path.insert(0, Point::new(x + 1, y, point.steps + 1));
		}
		if y > 0 && map[y - 1][x].can_go(&map[y][x], dir) {
			path.insert(0, Point::new(x, y - 1, point.steps + 1));
		}
		if y < map.len() - 1 && map[y + 1][x].can_go(&map[y][x], dir) {
			path.insert(0, Point::new(x, y + 1, point.steps + 1));
		}
		trail.insert(point);
	}
	None
}

fn main() {
	let file = File::open("./day12/input").unwrap();
	let lines = BufReader::new(file).lines();

	let mut map = Vec::new();
	for line in lines {
		map.push(line.unwrap().chars()
			.map(|id| Tile::new(id))
			.collect::<Vec<Tile>>());
	}

	// Part 1
	let start = find(&map, TileType::Start);
	println!("Steps to end: {}", walk(&map, start, &Direction::Uphill,
		|tile| tile.tile_type == TileType::End)
		.unwrap());

	// Part 2
	let end = find(&map, TileType::End);
	println!("Shortest path: {}", walk(&map, end, &Direction::Downhill,
		|tile| tile.level == 'a' as u8)
		.unwrap());
}
