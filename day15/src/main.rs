use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Hash)]
struct Position {
	x: i32,
	y: i32,
}

struct Sensor {
	position: Position,
	beacon: Position,
}

impl Position {
	fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}
	fn x_distance(&self, other: &Self) -> i32 {
		(self.x - other.x).abs()
	}
	fn y_distance(&self, other: &Self) -> i32 {
		(self.y - other.y).abs()
	}
	fn distance(&self, other: &Self) -> i32 {
		self.x_distance(other) + self.y_distance(other)
	}
}

impl Sensor {
	fn parse_value(val: &str) -> i32 {
		let mut chars = val.chars();
		let mut str = String::new();
		while let Some(char) = chars.next() {
			if !char.is_ascii_digit() && char != '-' {
				break;
			}
			str.push(char);
		}
		str.parse().unwrap()
	}
	fn parse(line: &str) -> Self {
		let sensor_x = Self::parse_value(&line[line.find("x=").unwrap() + 2..]);
		let sensor_y = Self::parse_value(&line[line.find("y=").unwrap() + 2..]);
		let beacon_x = Self::parse_value(&line[line.rfind("x=").unwrap() + 2..]);
		let beacon_y = Self::parse_value(&line[line.rfind("y=").unwrap() + 2..]);
		Self {
			position: Position::new(sensor_x, sensor_y),
			beacon: Position::new(beacon_x, beacon_y),
		}
	}
	fn radius(&self) -> i32 {
		self.position.distance(&self.beacon)
	}
	fn contains(&self, position: &Position) -> bool {
		self.radius() >= self.position.distance(position)
	}
}

fn get_non_beacon_count<'a>(row: i32, sensors: impl Iterator<Item=&'a Sensor>) -> usize {
	let in_range: Vec<_> = sensors
		.filter(|sensor| sensor.radius() >= sensor.position.y - row)
		.collect();

	let covered: HashSet<_> = in_range.iter()
		.filter(|sensor| sensor.position.y == row)
		.map(|sensor| sensor.position.x)
		.collect();

	in_range.iter()
		.map(|sensor| {
			let row_distance = (sensor.position.y - row).abs();
			let start = sensor.position.x - sensor.radius() + row_distance;
			let end = sensor.position.x + sensor.radius() - row_distance;
			(start..end)
				.map(|x| Position::new(x, row))
				.filter(|pos| !covered.contains(&pos.x))
		})
		.flatten()
		.collect::<HashSet<_>>()
		.len()
}

fn get_tuning_frequency<'a>(
	min_pos: i32, max_pos: i32, multiplier: u64,
	sensors: &Vec<Sensor>,
) -> Option<u64> {
	for sensor in sensors {
		let top = sensor.position.y - sensor.radius() - 1;
		let bottom = sensor.position.y + sensor.radius() + 1;
		for y in top..bottom {
			if y < min_pos || y > max_pos {
				continue;
			}
			let row_distance = (sensor.position.y - y).abs();
			let left = sensor.position.x - sensor.radius() - 1 + row_distance;
			let right = sensor.position.x + sensor.radius() + 1 - row_distance;
			if left < min_pos || left > max_pos || right < min_pos || right > max_pos {
				continue;
			}
			let left_pos = Position::new(left, y);
			if !sensors.iter().any(|sensor| sensor.contains(&left_pos)) {
				return Some(left_pos.x as u64 * multiplier + left_pos.y as u64);
			}
			let right_pos = Position::new(right, y);
			if !sensors.iter().any(|sensor| sensor.contains(&right_pos)) {
				return Some(right_pos.x as u64 * multiplier + right_pos.y as u64);
			}
		}
	}
	None
}

fn main() {
	let file = File::open("./day15/input").unwrap();
	let lines = BufReader::new(file).lines();
	let mut sensors = Vec::new();

	for result in lines {
		let line = result.unwrap();
		sensors.push(Sensor::parse(&line));
	}

	// Part 1
	let row = 2_000_000;
	println!("Non-beacon count: {}", get_non_beacon_count(row, sensors.iter()));

	// Part 2
	let min_pos = 0;
	let max_pos = 4_000_000;
	let multiplier = 4_000_000;
	println!("Tuning frequency: {}",
		get_tuning_frequency(min_pos, max_pos, multiplier, &sensors).unwrap());
}
