use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

fn main() {
	let file = File::open("./day04/input").unwrap();
	let lines = BufReader::new(file).lines();

	let mut full_overlap_count = 0_u32;
	let mut any_overlap_count = 0_u32;

	for result in lines {
		let line = result.unwrap();

		let ranges: Vec<Range<u32>> = line.split(',')
			.map(|range| range.split('-'))
			.map(|mut idx| (idx.next().unwrap().parse::<u32>().unwrap()
				..idx.next().unwrap().parse::<u32>().unwrap()))
			.collect();

		if has_full_overlap(&ranges[0], &ranges[1])
			|| has_full_overlap(&ranges[1], &ranges[0]) {
			full_overlap_count += 1;
			any_overlap_count += 1;
			continue;
		}

		if has_any_overlap(&ranges[0], &ranges[1])
			|| has_any_overlap(&ranges[1], &ranges[0]) {
			any_overlap_count += 1;
		}
	}

	// Part 1
	println!("Full overlaps: {}", full_overlap_count);

	// Part 2
	println!("Partial overlaps: {}", any_overlap_count);
}

fn has_full_overlap(range1: &Range<u32>, range2: &Range<u32>) -> bool {
	range1.start >= range2.start && range1.end <= range2.end
}

fn has_any_overlap(range1: &Range<u32>, range2: &Range<u32>) -> bool {
	range1.contains(&range2.start) || range1.contains(&range2.end)
}
