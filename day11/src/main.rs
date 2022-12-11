use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Item {
	worry_level: u64,
}

enum Operation {
	Add,
	Multiply,
}

enum Value {
	Old,
	Constant(u64),
}

struct Condition {
	divisible_by: u64,
	true_index: usize,
	false_index: usize,
}

struct Monkey {
	items: Vec<Item>,
	operation: (Value, Operation, Value),
	test: Condition,
	inspect_count: u32,
}

impl Item {
	fn new(worry_level: u64) -> Self {
		Self {
			worry_level,
		}
	}
}

impl Operation {
	fn get(&self, a: u64, b: u64) -> u64 {
		match self {
			Operation::Add => a + b,
			Operation::Multiply => a * b,
		}
	}
}

impl FromStr for Operation {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"+" => Ok(Operation::Add),
			"*" => Ok(Operation::Multiply),
			_ => Err(()),
		}
	}
}

impl Value {
	fn get(&self, old_value: u64) -> u64 {
		match self {
			Value::Old => old_value,
			Value::Constant(value) => *value,
		}
	}
}

impl FromStr for Value {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"old" => Ok(Value::Old),
			val => Ok(Value::Constant(val.parse::<u64>().unwrap())),
		}
	}
}

impl Condition {
	fn get(&self, value: u64) -> usize {
		if value % self.divisible_by == 0 {
			self.true_index
		} else {
			self.false_index
		}
	}
}

impl Monkey {
	fn parse(lines: &[String]) -> Monkey {
		let items: Vec<Item> = lines[0]["  Starting items: ".len()..]
			.split(", ")
			.map(|item| Item::new(item.parse::<u64>().unwrap()))
			.collect::<Vec<_>>()
			.into_iter()
			.rev()
			.collect();

		let operation_parts: Vec<&str> = lines[1]["  Operation: new = ".len()..]
			.split(' ')
			.collect();

		let operation = (
			operation_parts[0].parse().unwrap(),
			operation_parts[1].parse().unwrap(),
			operation_parts[2].parse().unwrap(),
		);

		let divisible_by = lines[2]["  Test: divisible by ".len()..]
			.parse::<u64>().unwrap();

		let true_index = lines[3]["    If true: throw to monkey ".len()..]
			.parse::<usize>().unwrap();

		let false_index = lines[4]["    If false: throw to monkey ".len()..]
			.parse::<usize>().unwrap();

		Monkey {
			items,
			operation,
			test: Condition {
				divisible_by,
				true_index,
				false_index,
			},
			inspect_count: 0,
		}
	}

	fn get_worry_level(&self, old_value: u64) -> u64 {
		let val1 = self.operation.0.get(old_value);
		let val2 = self.operation.2.get(old_value);
		self.operation.1.get(val1, val2)
	}

	fn throw(&mut self, worry_fn: impl Fn(u64) -> u64) -> Option<(Item, usize)> {
		let mut item = self.items.pop()?;
		self.inspect_count += 1;
		item.worry_level = worry_fn(self.get_worry_level(item.worry_level));
		let index = self.test.get(item.worry_level);
		Some((item, index))
	}

	fn collect(&mut self, item: Item) {
		self.items.insert(0, item);
	}

	fn get_divisor(&self) -> u64 {
		self.test.divisible_by
	}
}

fn get_monkeys(lines: &Vec<String>) -> Vec<Monkey> {
	let mut monkeys = Vec::new();
	for i in 0..(lines.len() as f32 / 7_f32).ceil() as usize {
		let start = i * 7;
		monkeys.push(Monkey::parse(&lines[start + 1..start + 6]))
	}
	monkeys
}

fn simulate(monkeys: &mut Vec<Monkey>, times: usize, worry_fn: impl Fn(u64) -> u64) {
	for _ in 0..times {
		for i in 0..monkeys.len() {
			while let Some(item) = monkeys[i].throw(&worry_fn) {
				monkeys[item.1].collect(item.0);
			}
		}
	}
}

fn get_monkey_business_level(monkeys: &mut Vec<Monkey>) -> u64 {
	monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
	monkeys[0..2].iter()
		.map(|monkey| monkey.inspect_count as u64)
		.product::<u64>()
}

fn main() {
	let file = File::open("./day11/input").unwrap();
	let lines: Vec<String> = BufReader::new(file).lines()
		.map(|line| line.unwrap())
		.collect();

	let mut monkeys1 = get_monkeys(&lines);
	simulate(&mut monkeys1, 20, |level| level / 3);

	let mut monkeys2 = get_monkeys(&lines);
	let divisor: u64 = monkeys2.iter()
		.map(|monkey| monkey.get_divisor())
		.product();
	simulate(&mut monkeys2, 10_000, |level| level % divisor);

	// Part 1
	println!("Monkey business (20): {}", get_monkey_business_level(&mut monkeys1));

	// Part 2
	println!("Monkey business (10 000): {}", get_monkey_business_level(&mut monkeys2));
}