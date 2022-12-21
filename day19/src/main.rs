use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Blueprint {
	id: u8,
	ore_robot_ore_cost: u16,
	clay_robot_ore_cost: u16,
	obsidian_robot_ore_cost: u16,
	obsidian_robot_clay_cost: u16,
	geode_robot_ore_cost: u16,
	geode_robot_obsidian_cost: u16,
}

#[derive(Copy, Clone)]
struct Resources {
	ore: u16,
	clay: u16,
	obsidian: u16,
	geode: u16,
	ore_robots: u16,
	clay_robots: u16,
	obsidian_robots: u16,
	geode_robots: u16,
}

impl FromStr for Blueprint {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let id = s["Blueprint ".len()..s.find(':').unwrap()]
			.parse::<u8>().unwrap();

		let parts: Vec<u16> = s.split(' ')
			.map(|part| part.parse::<u16>())
			.filter_map(|part| part.ok())
			.collect();

		Ok(Self {
			id,
			ore_robot_ore_cost: parts[0],
			clay_robot_ore_cost: parts[1],
			obsidian_robot_ore_cost: parts[2],
			obsidian_robot_clay_cost: parts[3],
			geode_robot_ore_cost: parts[4],
			geode_robot_obsidian_cost: parts[5],
		})
	}
}

impl Resources {
	fn new() -> Self {
		Self {
			ore: 0,
			clay: 0,
			obsidian: 0,
			geode: 0,
			ore_robots: 1,
			clay_robots: 0,
			obsidian_robots: 0,
			geode_robots: 0,
		}
	}

	fn mined(&self) -> Self {
		let mut clone = self.clone();
		clone.ore += clone.ore_robots;
		clone.clay += clone.clay_robots;
		clone.obsidian += clone.obsidian_robots;
		clone.geode += clone.geode_robots;
		clone
	}
}

fn build_geode_robot(blueprint: &Blueprint, resources: &Resources) -> Option<Resources> {
	if resources.ore < blueprint.geode_robot_ore_cost
		|| resources.obsidian < blueprint.geode_robot_obsidian_cost
	{
		return None;
	}

	let mut res = resources.mined();
	res.ore -= blueprint.geode_robot_ore_cost;
	res.obsidian -= blueprint.geode_robot_obsidian_cost;
	res.geode_robots += 1;
	Some(res)
}

fn build_obsidian_robot(blueprint: &Blueprint, resources: &Resources) -> Option<Resources> {
	if resources.ore < blueprint.obsidian_robot_ore_cost
		|| resources.clay < blueprint.obsidian_robot_clay_cost
		|| resources.obsidian_robots >= blueprint.geode_robot_obsidian_cost
	{
		return None;
	}

	let mut res = resources.mined();
	res.ore -= blueprint.obsidian_robot_ore_cost;
	res.clay -= blueprint.obsidian_robot_clay_cost;
	res.obsidian_robots += 1;
	Some(res)
}

fn build_clay_robot(blueprint: &Blueprint, resources: &Resources) -> Option<Resources> {
	if resources.ore < blueprint.clay_robot_ore_cost
		|| resources.clay_robots >= blueprint.obsidian_robot_clay_cost
	{
		return None;
	}

	let mut res = resources.mined();
	res.ore -= blueprint.clay_robot_ore_cost;
	res.clay_robots += 1;
	Some(res)
}

fn build_ore_robot(blueprint: &Blueprint, resources: &Resources) -> Option<Resources> {
	if resources.ore < blueprint.ore_robot_ore_cost
		|| resources.ore_robots >= max(blueprint.clay_robot_ore_cost,
		max(blueprint.obsidian_robot_ore_cost, blueprint.geode_robot_ore_cost))
	{
		return None;
	}

	let mut res = resources.mined();
	res.ore -= blueprint.ore_robot_ore_cost;
	res.ore_robots += 1;
	Some(res)
}

fn step(blueprint: &Blueprint, resources: Resources, remaining: u16) -> Vec<Resources> {
	if remaining == 0 {
		return vec![resources];
	}
	let next = remaining - 1;

	let res_build = if let Some(res) = build_geode_robot(blueprint, &resources) {
		Some(res)
	} else if let Some(res) = build_obsidian_robot(blueprint, &resources) {
		Some(res)
	} else if let Some(res) = build_clay_robot(blueprint, &resources) {
		Some(res)
	} else {
		None
	};

	let paths = [
		build_ore_robot(blueprint, &resources)
			.map(|res| step(blueprint, res, next)),
		res_build.map(|res| step(blueprint, res, next)),
		Some(step(blueprint, resources.mined(), next)),
	];

	paths.into_iter()
		.filter_map(|path| path)
		.flatten()
		.collect()
}

fn simulate(minutes: u16, blueprint: &Blueprint) -> u32 {
	step(blueprint, Resources::new(), minutes).into_iter()
		.map(|step| step.geode)
		.max()
		.unwrap() as u32
		* blueprint.id as u32
}

fn main() {
	let file = File::open("./day19/input").unwrap();
	let blueprints: Vec<Blueprint> = BufReader::new(file).lines()
		.map(|line| line.unwrap().parse::<Blueprint>().unwrap())
		.collect();

	// Part 1
	println!("Quality level: {}", blueprints.iter()
		.map(|blueprint| simulate(24, blueprint))
		.sum::<u32>())
}
