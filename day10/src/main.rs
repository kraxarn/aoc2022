use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use crate::Instruction::{AddX, NoOp};

enum Instruction {
	AddX(i32),
	NoOp,
}

impl Instruction {
	fn cycles(&self) -> i32 {
		match self {
			AddX(_) => 2,
			NoOp => 1,
		}
	}
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<_> = s.split(' ').collect();
		match parts[0] {
			"addx" => Ok(AddX(parts[1].parse::<i32>().unwrap())),
			"noop" => Ok(NoOp),
			_ => Err(()),
		}
	}
}

fn main() {
	let file = File::open("./day10/input").unwrap();
	let lines = BufReader::new(file).lines();

	let mut signal_sum = 0;
	let mut cycle = 0;
	let mut x = 1;

	let mut display = [['.'; 40]; 6];

	for result in lines {
		let line = result.unwrap();

		let instruction = line.parse::<Instruction>().unwrap();
		for _ in cycle..cycle + instruction.cycles() {
			let x_pos = cycle % 40;
			if (x - 1..=x + 1).contains(&x_pos) {
				let y_pos = cycle / 40;
				display[y_pos as usize][x_pos as usize] = '#';
			}

			cycle += 1;
			if (cycle - 20) % 40 == 0 {
				signal_sum += cycle * x;
			}
		}

		match instruction {
			AddX(ix) => x += ix,
			NoOp => {}
		}
	}

	// Part 1
	println!("Signal strength: {}", signal_sum);

	// Part 2
	for row in display {
		for column in row {
			print!("{}", column);
		}
		println!();
	}
}
