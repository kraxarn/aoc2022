use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;

#[derive(Eq, Debug)]
enum Packet {
	Integer(u32),
	List(Vec<Packet>),
}

impl PartialEq<Self> for Packet {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Packet::Integer(a), Packet::Integer(b)) => a == b,
			(Packet::List(a), Packet::List(b)) => a == b,
			(Packet::Integer(a), Packet::List(b)) =>
				&vec![Packet::Integer(*a)] == b,
			(Packet::List(a), Packet::Integer(b)) =>
				a == &vec![Packet::Integer(*b)],
		}
	}
}

impl PartialOrd for Packet {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Packet {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
			(Packet::List(a), Packet::List(b)) => a.cmp(&b),
			(Packet::Integer(a), Packet::List(b)) =>
				vec![Packet::Integer(*a)].cmp(b),
			(Packet::List(a), Packet::Integer(b)) =>
				a.cmp(&vec![Packet::Integer(*b)]),
		}
	}
}

fn parse_line(line: &str) -> Option<Packet> {
	parse(&mut line.chars().peekable())
}

fn parse(chars: &mut Peekable<impl Iterator<Item=char>>) -> Option<Packet> {
	match chars.next() {
		Some(char) if char.is_ascii_digit() => {
			let mut num = String::from(char);
			while chars.peek().is_some() && chars.peek().unwrap().is_ascii_digit() {
				num.push(chars.next().unwrap());
			}
			Some(Packet::Integer(num.parse::<u32>().unwrap()))
		},
		Some('[') => {
			let mut values = Vec::new();
			while let Some(value) = parse(chars) {
				values.push(value);
				if let Some('[') = chars.next() {
					break;
				}
			}
			Some(Packet::List(values))
		},
		_ => None,
	}
}

fn get_decoder_key(key: u32) -> Packet {
	Packet::List(vec![Packet::List(vec![Packet::Integer(key)])])
}

fn find_packet_index(packets: &Vec<&Packet>, packet: &Packet) -> Option<usize> {
	packets.iter()
		.enumerate()
		.find(|(_, p)| p == &&packet)
		.map(|(i, _)| i + 1)
}

fn main() {
	let file = File::open("./day13/input").unwrap();
	let lines = BufReader::new(file).lines();
	let mut pairs = Vec::new();
	let mut in_order = Vec::new();
	let mut index = 1;

	for result in lines {
		let line = result.unwrap();
		if let Some(value) = parse_line(&line) {
			pairs.push(value);
		} else {
			let left = &pairs[pairs.len() - 2];
			let right = &pairs[pairs.len() - 1];
			if left <= right {
				in_order.push(index);
			}
			index += 1;
		}
	}

	let divider1 = get_decoder_key(2);
	let divider2 = get_decoder_key(6);

	let mut packets: Vec<_> = pairs.iter()
		.chain([&divider1, &divider2])
		.collect();

	packets.sort();

	// Part 1
	println!("Sorted: {}", in_order.iter().sum::<u32>());

	// Part 2
	println!("Decoder key: {}",
		find_packet_index(&packets, &divider1).unwrap()
			* find_packet_index(&packets, &divider2).unwrap());
}
