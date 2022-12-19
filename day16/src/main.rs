use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Hash)]
struct Path {
	source: String,
	destination: String,
}

struct Valve {
	name: String,
	flow_rate: u32,
	mask: u64,
	tunnels: Vec<String>,
}

impl Path {
	fn new(src: &str, dest: &str) -> Self {
		Self {
			source: String::from(src),
			destination: String::from(dest),
		}
	}
}

impl Valve {
	fn parse(i: u32, line: &str) -> Self {
		let names: Vec<&str> = line.split(' ')
			.map(|part| &part[0..2])
			.filter(|part| part.chars().all(|char| char.is_ascii_uppercase()))
			.collect();

		let flow_rate_start = line.find("flow rate=").unwrap() + 10;
		let flow_rate_end = line.find(';').unwrap();
		let flow_rate = line[flow_rate_start..flow_rate_end].parse::<u32>().unwrap();

		let tunnels: Vec<String> = names[1..].iter()
			.map(|&name| String::from(name))
			.collect();

		Self {
			name: String::from(names[0]),
			mask: 2_u64.pow(i),
			flow_rate,
			tunnels,
		}
	}
}

fn find_paths(valves: &HashMap<String, Valve>) -> HashMap<Path, Option<u32>> {
	let mut paths = HashMap::new();

	for src in valves.keys() {
		for dest in valves.keys() {
			let path = Path::new(src, dest);
			let distance = if valves[src].tunnels.contains(dest) {
				Some(1)
			} else {
				None
			};
			paths.insert(path, distance);
		}
	}

	for x in valves.keys() {
		for y in valves.keys() {
			for z in valves.keys() {
				let path = Path::new(y, z);
				let yz = paths[&path].unwrap_or(u32::MAX);
				let yx = paths[&Path::new(y, x)];
				let xz = paths[&Path::new(x, z)];
				let distance = match (yx, xz) {
					(Some(a), Some(b)) => min(yz as u64, a as u64 + b as u64) as u32,
					(_, _) => yz,
				};
				paths.insert(path, Some(distance));
			}
		}
	}

	paths
}

fn walk(
	valve: String,
	minutes: u32,
	state: u64,
	valves: &HashMap<String, Valve>,
	paths: &HashMap<Path, Option<u32>>,
	pressure: u32,
	result: &mut HashMap<u64, u32>,
) {
	let new_state = *result.get(&state).unwrap_or(&0);
	result.insert(state, max(new_state, pressure));

	for (dest_name, dest) in valves {
		if dest.flow_rate == 0 {
			continue;
		}
		let mask = valves[dest_name].mask;
		let dist = paths[&Path::new(&valve, dest_name)].unwrap();
		let new_minutes = minutes as i32 - dist as i32 - 1;
		if (state & mask) > 0 || new_minutes < 0 {
			continue;
		}
		let flow_rate = valves[dest_name].flow_rate;
		let new_pressure = pressure + (new_minutes as u32 * flow_rate);

		walk(String::from(dest_name), new_minutes as u32,
			state | mask, valves, paths, new_pressure, result);
	}
}

fn get_max_sum(results: &HashMap<u64, u32>) -> u32 {
	let mut result = 0;
	for (state1, flow1) in results {
		for (state2, flow2) in results {
			if (state1 & state2) == 0 {
				result = max(result, flow1 + flow2);
			}
		}
	}
	result
}

fn main() {
	let file = File::open("./day16/input").unwrap();
	let lines = BufReader::new(file).lines();

	let mut i = 0;
	let mut valves = HashMap::new();

	for result in lines {
		let line = result.unwrap();
		let valve = Valve::parse(i, &line);
		valves.insert(String::from(&valve.name), valve);
		i += 1;
	}

	let paths = find_paths(&valves);

	// Part 1
	let mut results = HashMap::new();
	walk(String::from("AA"), 30, 0, &valves, &paths, 0, &mut results);
	println!("Max alone flow rate: {}", results.values().max().unwrap());

	// Part 2
	results.clear();
	walk(String::from("AA"), 26, 0, &valves, &paths, 0, &mut results);
	println!("Max elephant flow rate: {}", get_max_sum(&results));
}
