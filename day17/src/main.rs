use std::cmp::max;
use std::collections::HashSet;
use std::fs::read_to_string;

type Shape = Vec<Position>;

#[derive(Eq, PartialEq, Hash)]
struct Position {
	x: u64,
	y: u64,
}

struct Rock<'a> {
	position: Position,
	shape: &'a Shape,
	width: u64,
	height: u64,
}

enum Direction {
	Left,
	Right,
	Down,
}

impl Position {
	fn new(x: u64, y: u64) -> Self {
		Self { x, y }
	}

	fn to(&self, dir: &Direction) -> Self {
		match dir {
			Direction::Left => Position::new(self.x - 1, self.y),
			Direction::Right => Position::new(self.x + 1, self.y),
			Direction::Down => Position::new(self.x, self.y - 1),
		}
	}
}

impl<'a> Rock<'a> {
	fn new(shape: &'a Shape) -> Self {
		Self {
			position: Position::new(0, 0),
			shape,
			width: shape.iter().map(|pos| pos.x).max().unwrap() + 1,
			height: shape.iter().map(|pos| pos.y).max().unwrap() + 1,
		}
	}

	fn positions(&self, origin: &Position) -> Vec<Position> {
		self.shape.iter()
			.map(|pos| Position::new(origin.x + pos.x, origin.y - pos.y))
			.collect()
	}

	fn can_move(&self, dir: &Direction, points: &HashSet<Position>) -> bool {
		if match dir {
			Direction::Left => self.position.x == 0,
			Direction::Right => self.position.x + self.width > 6,
			Direction::Down => self.position.y <= self.height,
		} {
			false
		} else {
			let origin = self.position.to(dir);
			self.positions(&origin).iter().all(|pos| !points.contains(pos))
		}
	}

	fn try_move(&mut self, dir: &Direction, points: &HashSet<Position>) -> bool {
		if self.can_move(dir, points) {
			self.position = self.position.to(dir);
			true
		} else {
			false
		}
	}
}

impl Direction {
	fn parse(value: &char) -> Option<Self> {
		match value {
			'<' => Some(Direction::Left),
			'>' => Some(Direction::Right),
			_ => None,
		}
	}
}

fn get_all_shapes() -> [Shape; 5] {
	[
		// ####
		vec![
			Position::new(0, 0),
			Position::new(1, 0),
			Position::new(2, 0),
			Position::new(3, 0),
		],
		// .#.
		// ###
		// .#.
		vec![
			Position::new(1, 0),
			Position::new(0, 1),
			Position::new(1, 1),
			Position::new(2, 1),
			Position::new(1, 2),
		],
		// ..#
		// ..#
		// ###
		vec![
			Position::new(2, 0),
			Position::new(2, 1),
			Position::new(0, 2),
			Position::new(1, 2),
			Position::new(2, 2),
		],
		// #
		// #
		// #
		// #
		vec![
			Position::new(0, 0),
			Position::new(0, 1),
			Position::new(0, 2),
			Position::new(0, 3),
		],
		// ##
		// ##
		vec![
			Position::new(0, 0),
			Position::new(1, 0),
			Position::new(0, 1),
			Position::new(1, 1),
		],
	]
}

fn simulate(limit: usize, shapes: &[Shape], directions: &Vec<Direction>) -> u64 {
	let mut last_rock: Option<Rock> = None;
	let mut points = HashSet::new();
	let mut rock_count = 0;
	let mut push_count = 0;
	let mut height = 0;

	loop {
		let direction_index = push_count % directions.len();
		let shape_index = rock_count % shapes.len();

		if let Some(rock) = &mut last_rock {
			rock.try_move(&directions[direction_index], &points);
			push_count += 1;
			if rock.try_move(&Direction::Down, &points) {
				continue;
			}

			height = max(height, rock.position.y);
			let rock_positions = rock.positions(&rock.position);
			for position in rock_positions {
				points.insert(position);
			}
		}

		if rock_count >= limit {
			break;
		}

		let mut rock = Rock::new(&shapes[shape_index]);
		rock.position = Position::new(2, height + rock.height + 3);
		last_rock = Some(rock);
		rock_count += 1;
	}

	height
}

fn main() {
	let directions: Vec<Direction> = read_to_string("./day17/input")
		.unwrap()
		.trim_end()
		.chars()
		.map(|char| Direction::parse(&char).unwrap())
		.collect();

	let shapes = get_all_shapes();

	// Part 1
	println!("Short tower height: {}", simulate(2022, &shapes, &directions));
}
