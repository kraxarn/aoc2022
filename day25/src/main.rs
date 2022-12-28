use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_snafu(value: char) -> i64 {
	match value {
		'2' => 2,
		'1' => 1,
		'0' => 0,
		'-' => -1,
		'=' => -2,
		_ => panic!("Unknown char"),
	}
}

fn from_snafu(value: &str) -> i64 {
	value.chars()
		.map(|char| parse_snafu(char))
		.rev()
		.enumerate()
		.fold(0, |num, (i, digit)| num + (digit * 5_i64.pow(i as u32)))
}

fn to_snafu(value: i64) -> String {
	let mut remaining = value;
	let mut result = String::new();
	while remaining > 0 {
		let str = match remaining % 5 {
			0 => '0',
			1 => '1',
			2 => '2',
			3 => '=',
			4 => '-',
			_ => panic!("Unexpected result")
		};
		remaining = (remaining - parse_snafu(str)) / 5;
		result.insert(0, str);
	}
	result
}

fn main() {
	let file = File::open("./day25/input").unwrap();
	let lines: Vec<String> = BufReader::new(file).lines()
		.filter_map(|line| line.ok())
		.collect();

	let sum = lines.iter()
		.map(|line| from_snafu(line))
		.sum::<i64>();

	// Part 1
	println!("Sum: {}", to_snafu(sum));
}
