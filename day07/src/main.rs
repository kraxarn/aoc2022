use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("./day07/input").unwrap();

	let lines: Vec<String> = BufReader::new(file).lines()
		.map(|result| result.unwrap())
		.collect();

	let mut current = String::from("/");
	let mut directories = HashMap::new();

	for i in 0..lines.len() {
		let line = &lines[i];

		if !line.starts_with('$') || line == "$ cd /" {
			continue;
		}

		let args: Vec<&str> = line.split(' ').collect();

		match args[1] {
			"cd" if args[2] == ".." => {
				current = current[0..current.rfind("/").unwrap()].to_string();
			},
			"cd" => {
				current += &format!("{}/", &args[2]);
			},
			"ls" => {
				let mut dir_size = 0;
				for j in i + 1..lines.len() {
					let line = &lines[j];
					if let Ok(size) = line[..line.find(' ').unwrap()].parse::<u32>() {
						dir_size += size;
					} else if line.starts_with("$") {
						break;
					}
				}
				directories.insert(current.to_string(), dir_size);
			},
			arg => panic!("Unknown command: {}", arg),
		}
	}

	let mut total_sum = 0;
	let mut smallest_large = 0;

	let used_space = directories.iter()
		.map(|dir| dir.1)
		.sum::<u32>();

	let needed_space = (70_000_000 - used_space as i32 - 30_000_000).abs() as u32;

	for directory in &directories {
		let dir_size = directories.iter()
			.filter(|dir| (*dir).0.starts_with(directory.0))
			.map(|dir| dir.1)
			.sum::<u32>();

		if dir_size <= 100_000 {
			total_sum += dir_size;
		}

		if dir_size >= needed_space && (smallest_large == 0 || dir_size < smallest_large) {
			smallest_large = dir_size;
		}
	}

	// Part 1
	println!("Small directories size: {}", total_sum);

	// Part 2
	println!("Smallest large directory: {}", smallest_large);
}
