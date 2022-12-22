use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

type Monkeys = HashMap<String, Monkey>;

enum Operation {
	Add,
	Subtract,
	Multiply,
	Divide,
}

enum MonkeyJob {
	MathOperation(String, Operation, String),
	Number(i64),
}

struct Monkey {
	name: String,
	job: MonkeyJob,
}

impl Operation {
	fn calc(&self, a: i64, b: i64) -> i64 {
		match self {
			Operation::Add => a + b,
			Operation::Subtract => a - b,
			Operation::Multiply => a * b,
			Operation::Divide => a / b,
		}
	}
}

impl FromStr for Operation {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"+" => Ok(Operation::Add),
			"-" => Ok(Operation::Subtract),
			"*" => Ok(Operation::Multiply),
			"/" => Ok(Operation::Divide),
			_ => Err(()),
		}
	}
}

impl FromStr for MonkeyJob {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<&str> = s.split(' ').collect();
		match parts[..] {
			[name1, op, name2] => {
				Ok(MonkeyJob::MathOperation(
					String::from(name1),
					op.parse().unwrap(),
					String::from(name2),
				))
			},
			[num] => {
				Ok(MonkeyJob::Number(num.parse().unwrap()))
			},
			_ => Err(()),
		}
	}
}

impl FromStr for Monkey {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let sep = s.find(':').unwrap();
		Ok(Self {
			name: String::from(&s[0..sep]),
			job: s[sep + 2..].parse()?,
		})
	}
}

fn get_value(name: &str, monkeys: &Monkeys) -> i64 {
	match &monkeys[name].job {
		MonkeyJob::MathOperation(name1, op, name2) => {
			let val1 = get_value(name1, monkeys);
			let val2 = get_value(name2, monkeys);
			op.calc(val1, val2)
		}
		MonkeyJob::Number(num) => *num,
	}
}

fn main() {
	let file = File::open("./day21/input").unwrap();
	let monkeys: Monkeys = BufReader::new(file).lines()
		.map(|line| line.unwrap().parse::<Monkey>().unwrap())
		.map(|monkey| (String::from(&monkey.name), monkey))
		.collect();

	// Part 1
	println!("Root: {}", get_value("root", &monkeys));
}
