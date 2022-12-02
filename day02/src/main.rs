use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Choice {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

enum Result {
	Loss = 0,
	Draw = 3,
	Win = 6,
}

fn main() {
	let file = File::open("./day02/input").unwrap();
	let lines = BufReader::new(file).lines();

	let mut score1 = 0_u32;
	let mut score2 = 0_u32;

	for result in lines {
		let line = result.unwrap();
		let mut chars = line.chars();

		let char1 = chars.next().unwrap();
		chars.next().unwrap();
		let char2 = chars.next().unwrap();

		let opponent = parse_choice(char1);
		let me1 = parse_choice(char2);

		score1 += get_score(opponent, me1) as u32 + me1 as u32;

		let result = parse_result(char2);
		let me2 = get_choice(opponent, result);

		score2 += get_score(opponent, me2) as u32 + me2 as u32;
	}

	// Part 1
	println!("Score 1: {}", score1);

	// Part 2
	println!("Score 2: {}", score2);
}

fn parse_choice(value: char) -> Choice {
	match value {
		'A' | 'X' => Choice::Rock,
		'B' | 'Y' => Choice::Paper,
		'C' | 'Z' => Choice::Scissors,
		_ => panic!("Invalid choice: {}", value),
	}
}

fn parse_result(value: char) -> Result {
	match value {
		'X' => Result::Loss,
		'Y' => Result::Draw,
		'Z' => Result::Win,
		_ => panic!("Invalid result: {}", value),
	}
}

fn get_score(opponent: Choice, me: Choice) -> Result {
	if opponent == me {
		Result::Draw
	} else if (opponent as isize - me as isize).abs() == 1 {
		if opponent as isize > me as isize { Result::Loss } else { Result::Win }
	} else {
		if opponent as isize > me as isize { Result::Win } else { Result::Loss }
	}
}

fn get_choice(opponent: Choice, result: Result) -> Choice {
	let my_choice_index = opponent as i32 + match result {
		Result::Loss => -1,
		Result::Draw => 0,
		Result::Win => 1,
	};

	match my_choice_index {
		1 | 4 => Choice::Rock,
		2 => Choice::Paper,
		3 | 0 => Choice::Scissors,
		_ => panic!("Unknown index: {}", my_choice_index),
	}
}