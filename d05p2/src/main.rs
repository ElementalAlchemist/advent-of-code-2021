use std::collections::HashMap;
use std::fs;

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
	x: i32,
	y: i32,
}

impl Coordinate {
	fn new(val: &str) -> Self {
		let mut values = val.split(',');
		let x = values.next().unwrap().parse().unwrap();
		let y = values.next().unwrap().parse().unwrap();
		Self { x, y }
	}
}

struct Line {
	start: Coordinate,
	end: Coordinate,
}

fn main() {
	let line_coords: Vec<Line> = fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(|s| {
			let mut parts = s.split(" -> ");
			let start = parts.next().unwrap();
			let end = parts.next().unwrap();
			Line {
				start: Coordinate::new(start),
				end: Coordinate::new(end),
			}
		})
		.collect();

	let mut covered_coords: HashMap<Coordinate, usize> = HashMap::new();

	for line in line_coords.iter() {
		let start = &line.start;
		let end = &line.end;
		let horizontal = start.x == end.x;
		let vertical = start.y == end.y;
		if horizontal {
			let begin = start.y.min(end.y);
			let stop = start.y.max(end.y);
			for y in begin..=stop {
				let current_coord = Coordinate { x: start.x, y };
				*covered_coords.entry(current_coord).or_default() += 1;
			}
		} else if vertical {
			let begin = start.x.min(end.x);
			let stop = start.x.max(end.x);
			for x in begin..=stop {
				let current_coord = Coordinate { x, y: start.y };
				*covered_coords.entry(current_coord).or_default() += 1;
			}
		} else {
			let x_range = start.x.min(end.x)..=start.x.max(end.x);
			let x_range: Vec<i32> = if start.x > end.x {
				x_range.rev().collect()
			} else {
				x_range.collect()
			};
			let y_range = start.y.min(end.y)..=start.y.max(end.y);
			let y_range: Vec<i32> = if start.y > end.y {
				y_range.rev().collect()
			} else {
				y_range.collect()
			};
			let mut x_iter = x_range.iter();
			let mut y_iter = y_range.iter();
			loop {
				let curr_x = x_iter.next();
				let curr_y = y_iter.next();
				if curr_x.is_none() || curr_y.is_none() {
					break;
				}
				let curr_x = curr_x.unwrap();
				let curr_y = curr_y.unwrap();
				let current_coord = Coordinate { x: *curr_x, y: *curr_y };
				*covered_coords.entry(current_coord).or_default() += 1;
			}
		}
	}

	let dangerous_spots = covered_coords.values().filter(|x| **x > 1).count();
	println!("{}", dangerous_spots);
}
