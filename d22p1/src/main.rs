use std::collections::HashSet;
use std::fs;

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
	x: i32,
	y: i32,
	z: i32,
}

struct Step {
	activate: bool,
	x_min: i32,
	x_max: i32,
	y_min: i32,
	y_max: i32,
	z_min: i32,
	z_max: i32,
}

fn main() {
	let steps: Vec<Step> = {
		let mut steps: Vec<Step> = Vec::new();
		for input_line in fs::read_to_string("input.txt")
			.unwrap()
			.lines()
			.filter(|s| !s.is_empty())
		{
			let mut parts = input_line.split(' ');
			let activate = parts.next().unwrap() == "on";
			let mut coord_parts = parts.next().unwrap().split(',');
			let x_range = coord_parts.next().unwrap();
			let y_range = coord_parts.next().unwrap();
			let z_range = coord_parts.next().unwrap();

			let mut x_range = x_range.split('=').nth(1).unwrap().split("..");
			let x_min: i32 = x_range.next().unwrap().parse().unwrap();
			let x_max: i32 = x_range.next().unwrap().parse().unwrap();
			let mut y_range = y_range.split('=').nth(1).unwrap().split("..");
			let y_min: i32 = y_range.next().unwrap().parse().unwrap();
			let y_max: i32 = y_range.next().unwrap().parse().unwrap();
			let mut z_range = z_range.split('=').nth(1).unwrap().split("..");
			let z_min: i32 = z_range.next().unwrap().parse().unwrap();
			let z_max: i32 = z_range.next().unwrap().parse().unwrap();

			steps.push(Step {
				activate,
				x_min,
				x_max,
				y_min,
				y_max,
				z_min,
				z_max,
			});
		}
		steps
	};

	let mut active_cubes: HashSet<Coordinate> = HashSet::new();
	for step in steps.iter() {
		for x in step.x_min..=step.x_max {
			if x < -50 || x > 50 {
				continue;
			}
			for y in step.y_min..=step.y_max {
				if y < -50 || y > 50 {
					continue;
				}
				for z in step.z_min..=step.z_max {
					if z < -50 || z > 50 {
						continue;
					}
					if step.activate {
						active_cubes.insert(Coordinate { x, y, z });
					} else {
						active_cubes.remove(&Coordinate { x, y, z });
					}
				}
			}
		}
	}

	println!("{}", active_cubes.len());
}
