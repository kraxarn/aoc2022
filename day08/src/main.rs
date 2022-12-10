use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("./day08/input").unwrap();
	let lines = BufReader::new(file).lines();

	let mut trees = Vec::new();

	for result in lines {
		let line = result.unwrap();
		trees.push(line.chars().collect::<Vec<char>>());
	}

	let width = get_width(&trees);
	let height = get_height(&trees);
	let mut visible = get_result_vec(width, height);

	for x in 1..width - 1 {
		for y in get_y(&trees, x, 0, false) {
			visible[y][x] = true;
		}
	}
	for x in (1..width - 1).rev() {
		for y in get_y(&trees, x, height - 1, true) {
			visible[y][x] = true;
		}
	}
	for y in 1..height - 1 {
		for x in get_x(&trees, 0, y, false) {
			visible[y][x] = true;
		}
	}
	for y in (1..height - 1).rev() {
		for x in get_x(&trees, width - 1, y, true) {
			visible[y][x] = true;
		}
	}

	let mut max_score = 0;

	for x in 1..width - 1 {
		for y in 1..height - 1 {
			let current = trees[y][x];

			let mut top_score = 0;
			for cy in (0..y).rev() {
				top_score += 1;
				if trees[cy][x] >= current {
					break;
				}
			}

			let mut left_score = 0;
			for cx in (0..x).rev() {
				left_score += 1;
				if trees[y][cx] >= current {
					break;
				}
			}

			let mut bottom_score = 0;
			for cy in y + 1..height {
				bottom_score += 1;
				if trees[cy][x] >= current {
					break;
				}
			}

			let mut right_score = 0;
			for cx in x + 1..width {
				right_score += 1;
				if trees[y][cx] >= current {
					break;
				}
			}

			let score = top_score * left_score * bottom_score * right_score;
			if score > max_score {
				max_score = score;
			}
		}
	}

	// Part 1
	println!("Visible trees: {}", get_count(&visible));

	// Part 2
	println!("Max score: {}", max_score);
}

fn get_width(grid: &Vec<Vec<char>>) -> usize {
	grid.len()
}

fn get_height(grid: &Vec<Vec<char>>) -> usize {
	grid[0].len()
}

fn get_range(pos: usize, size: usize, reverse: bool) -> Vec<usize> {
	if reverse {
		(1..=pos).rev().collect()
	} else {
		(pos..size).collect()
	}
}

fn get_y(input: &Vec<Vec<char>>, x: usize, y: usize, reverse: bool) -> Vec<usize> {
	let mut points = Vec::new();
	let range = get_range(y, get_height(&input), reverse);
	let mut current = '0';

	for y in range {
		if input[y][x] > current {
			points.push(y);
		}
		current = max(current, input[y][x]);
	}

	points
}

fn get_x(input: &Vec<Vec<char>>, x: usize, y: usize, reverse: bool) -> Vec<usize> {
	let mut points = Vec::new();
	let range = get_range(x, get_width(&input), reverse);
	let mut current = '0';

	for x in range {
		if input[y][x] > current {
			points.push(x);
		}
		current = max(current, input[y][x]);
	}

	points
}

fn get_count(grid: &Vec<Vec<bool>>) -> usize {
	grid.iter()
		.flatten()
		.filter(|&&v| v)
		.count()
}

fn get_result_vec(x: usize, y: usize) -> Vec<Vec<bool>> {
	let mut vec = Vec::new();
	for _ in 0..x {
		vec.push((0..y).map(|_| false).collect::<Vec<bool>>());
	}
	for x in 0..x {
		vec[0][x] = true;
		vec[y - 1][x] = true;
	}
	for y in 0..y {
		vec[y][0] = true;
		vec[y][x - 1] = true;
	}
	vec
}